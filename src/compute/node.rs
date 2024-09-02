use crate::state::State;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Node {
    Literal(f32),
    // UnaryOp(UnaryOp, Node),
    BinaryOp(BinaryOp, Box<Node>, Box<Node>),
    // Function(Function, Vec<Node>),
    Reference(String),
}

impl Node {
    pub fn get_references(&self) -> Vec<&String> {
        match self {
            Node::Reference(reference) => vec![reference],
            Node::BinaryOp(_, left, right) => {
                let mut left_refs = left.get_references();
                let mut right_refs = right.get_references();

                left_refs.append(&mut right_refs);

                left_refs
            }
            _ => vec![],
        }
    }

    pub fn compute(self, state: &State) -> Result<f32, String> {
        match self {
            Node::Literal(num) => Ok(num),
            Node::BinaryOp(op, left, right) => {
                let left_result = (*left).compute(state);
                let right_result = (*right).compute(state);

                let Ok(x) = left_result else {
                    return Err(left_result.unwrap_err());
                };

                let Ok(y) = right_result else {
                    return Err(right_result.unwrap_err());
                };

                if y == 0.0 && op == BinaryOp::Divide {
                    return Err("Divide by 0 error".to_string());
                }

                Ok(match op {
                    BinaryOp::Add => x + y,
                    BinaryOp::Subtract => x - y,
                    BinaryOp::Multiply => x * y,
                    BinaryOp::Divide => x / y,
                })
            }
            Node::Reference(reference) => {
                let addr_result = resolve_reference(&reference);
                let Ok(addr) = addr_result else {
                    return Err(addr_result.unwrap_err());
                };

                let cell = state.get_at(addr);

                if cell.value.is_empty() {
                    return Err(format!("Error: Empty value @ {}", reference));
                }

                if !cell.computed.is_computed {
                    panic!("REFERENCE IS NOT COMPUTED @ {} {:?}", reference, addr);
                }

                if cell.computed.error {
                    return Err(format!(
                        "Err @ {}: \"{}\"",
                        reference, cell.computed.display
                    ));
                }

                Ok(cell.computed.value.unwrap())
            }
        }
    }
}

pub enum UnaryOp {
    Negative,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum Function {
    Sum,
    Avg,
    Pow,
}

pub fn resolve_reference(reference: &String) -> Result<(u16, u16), String> {
    let re = Regex::new(r"^([A-Z]+)(\d+)$").unwrap();

    let Some(captures) = re.captures(&reference) else {
        return Err(format!("Could not parse reference: {}", reference));
    };

    let mut column = 0;
    for digit in captures[1].chars() {
        column = column * 26 + ((digit as u8) - b'A' + 1);
    }

    let Ok(row) = (&captures[2]).parse::<u16>() as Result<u16, _> else {
        // I think this should only happen on overflow?
        return Err(format!(
            "Could not parse column for reference: {}",
            reference
        ));
    };

    Ok((row - 1, column as u16 - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_references() {
        assert_eq!(
            resolve_reference(&"123".to_string()),
            Err("Could not parse reference: 123".to_string())
        );
        assert_eq!(resolve_reference(&"A1".to_string()), Ok((0, 0)));
        assert_eq!(resolve_reference(&"AA11".to_string()), Ok((10, 26)));
    }
}
