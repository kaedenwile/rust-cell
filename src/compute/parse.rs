use crate::compute::node::*;

// Parse a cell value (string) into a Node
pub fn parse(cell: &str) -> Result<Node, String> {
    // STEP 1: SPLIT STRING INTO TERMS
    let terms = split_into_terms(cell);

    // STEP 2: REDUCE TERMS INTO TREE
    // order of operations
    // 2.1 Parens & Functions
    // 2.2 Mult / Divide
    // 2.2 Add / Sub

    let paren_stack = match reduce_paren_stack(terms) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let terms = match paren_stack {
        ParenStack::Parens(terms) => terms,
        _ => panic!("Internal err"),
    };

    match make_node(terms) {
        Ok(node) => Ok(node),
        Err(err) => Err(err),
    }
}

fn split_into_terms(cell: &str) -> Vec<String> {
    let mut current_term = String::from("");
    let mut terms: Vec<String> = Vec::new();
    for char in cell.chars() {
        match char {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !current_term.trim().is_empty() {
                    terms.push(current_term.trim().to_string());
                    current_term = String::from("");
                }

                terms.push(char.to_string());
            }
            _ => current_term.push(char),
        }
    }
    if !current_term.trim().is_empty() {
        terms.push(current_term.trim().to_string());
    }

    terms
}

#[derive(Debug, PartialEq)]
enum ParenStack {
    Term(String),
    Parens(Vec<ParenStack>),
}

fn reduce_paren_stack(terms: Vec<String>) -> Result<ParenStack, String> {
    let mut root = ParenStack::Parens(Vec::new());
    let mut depth = 0;

    fn push(root: &mut ParenStack, depth: i32, term: ParenStack) {
        let mut target = root;
        for _ in 0..depth {
            match target {
                ParenStack::Parens(ref mut vec) => {
                    target = vec.last_mut().unwrap();
                }
                _ => panic!("Internal err!"),
            }
        }

        match target {
            ParenStack::Parens(ref mut vec) => vec.push(term),
            _ => panic!("Internal err!"),
        }
    }

    for term in terms.into_iter() {
        match term.trim() {
            "(" => {
                push(&mut root, depth, ParenStack::Parens(Vec::new()));
                depth += 1;
            }
            ")" => {
                if depth <= 0 {
                    return Err("Unmatched closing paren".to_string());
                }
                depth -= 1;
            }
            _ => push(&mut root, depth, ParenStack::Term(term.to_string())),
        }
    }
    if depth > 0 {
        return Err("Unmatched opening paren".to_string());
    }

    Ok(root)
}

fn make_node(raw_terms: Vec<ParenStack>) -> Result<Node, String> {
    enum Computed {
        Computed(Node),
        Raw(String),
    }

    // Reduce parentheses, parse numbers, parse refs
    let mut terms: Vec<Computed> = Vec::new();
    for x in raw_terms.into_iter() {
        terms.push(match x {
            ParenStack::Term(term) if resolve_reference(&term).is_ok() => {
                Computed::Computed(Node::Reference(term))
            }
            ParenStack::Term(term) => match term.parse::<f32>() {
                Ok(f) => Computed::Computed(Node::Literal(f)),
                Err(_) => Computed::Raw(term),
            },
            ParenStack::Parens(terms) => Computed::Computed(match make_node(terms) {
                Ok(node) => node,
                Err(e) => return Err(e),
            }),
        })
    }

    fn reduce_binary<F>(terms: &mut Vec<Computed>, f: F) -> Result<(), String>
    where
        F: Fn(&str) -> Option<BinaryOp>,
    {
        let mut cursor: usize = 0;
        while cursor < terms.len() {
            let op = match &terms[cursor] {
                Computed::Raw(op) if f(op).is_some() => op.to_string(),
                _ => {
                    cursor += 1;
                    continue;
                }
            };

            if cursor < 1 || cursor >= terms.len() - 1 {
                return Err("BinaryOp at boundary".to_string());
            }

            let prev = match terms.remove(cursor - 1) {
                Computed::Raw(_) => return Err("Bad multiplication after raw".to_string()),
                Computed::Computed(node) => node,
            };
            let next = match terms.remove(cursor) {
                Computed::Raw(_) => return Err("Bad multiplication before raw".to_string()),
                Computed::Computed(node) => node,
            };

            terms[cursor - 1] = Computed::Computed(Node::BinaryOp(
                f(op.as_str()).unwrap(),
                Box::new(prev),
                Box::new(next),
            ));
        }
        Ok(())
    }

    // Reduce multiplication and division
    if let Err(e) = reduce_binary(&mut terms, |op| match op {
        "*" => Some(BinaryOp::Multiply),
        "/" => Some(BinaryOp::Divide),
        _ => None,
    }) {
        return Err(e);
    }

    // Reduce addition and subtraction
    if let Err(e) = reduce_binary(&mut terms, |op| match op {
        "+" => Some(BinaryOp::Add),
        "-" => Some(BinaryOp::Subtract),
        _ => None,
    }) {
        return Err(e);
    }

    if (&terms).len() != 1 {
        Err("Could not fully reduce".to_string())
    } else {
        match terms.into_iter().next().unwrap() {
            Computed::Raw(_) => Err("Could not reduce from raw".to_string()),
            Computed::Computed(node) => Ok(node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! paren {
        ( $( $x:expr ),+ ) => {{
            let mut v = Vec::new();

            $(
                v.push(term!($x));
            )*

            ParenStack::Parens(v)
        }};
    }

    macro_rules! term {
        ( $val:literal ) => {
            ParenStack::Term($val.to_string())
        };
        ( $x:expr  ) => {
            $x
        };
    }

    #[test]
    fn test_split_into_terms() {
        assert_eq!(split_into_terms("123"), vec!["123"]);
        assert_eq!(
            split_into_terms("1 + 2 * 3 - 4 / 5"),
            vec!["1", "+", "2", "*", "3", "-", "4", "/", "5"]
        );
        assert_eq!(
            split_into_terms("( 1.5 * (1.2 - B3 ) + A4)"),
            vec!["(", "1.5", "*", "(", "1.2", "-", "B3", ")", "+", "A4", ")"]
        );
    }

    #[test]
    fn test_reduce_paren_stack() {
        let reduce_paren_stack =
            |v: Vec<&str>| reduce_paren_stack(v.iter().map(|s| s.to_string()).collect());

        assert_eq!(
            reduce_paren_stack(vec!["1", "+", "2"]),
            Ok(paren!("1", "+", "2")) // Ok(ParenStack::Parens(vec![term("1"), term("+"), term("2")]))
        );

        assert_eq!(
            reduce_paren_stack(vec!["3", "*", "(", "1", "+", "2", ")"]),
            Ok(paren!("3", "*", paren!("1", "+", "2")))
        );

        assert_eq!(
            reduce_paren_stack(vec![
                "(", "B4", "-", "(", "C4", "/", "A2", ")", ")", "*", "(", "1", "+", "2", ")"
            ]),
            Ok(paren!(
                paren!("B4", "-", paren!("C4", "/", "A2")),
                "*",
                paren!("1", "+", "2")
            ))
        );

        assert_eq!(
            reduce_paren_stack(vec!["3", "*", "(", "1", "+", "2"]),
            Err("Unmatched opening paren".to_string())
        );

        assert_eq!(
            reduce_paren_stack(vec!["3", "*", "(", "1", "+", "2", ")", ")"]),
            Err("Unmatched closing paren".to_string())
        );
    }

    #[test]
    fn test_make_node() {
        let easy_make_node = |ast: ParenStack| match ast {
            ParenStack::Term(t) => make_node(vec![ParenStack::Term(t)]),
            ParenStack::Parens(terms) => make_node(terms),
        };

        assert_eq!(easy_make_node(paren!("1")), Ok(Node::Literal(1.0)));
        assert_eq!(
            easy_make_node(paren!("1", "+", "2")),
            Ok(Node::BinaryOp(
                BinaryOp::Add,
                Box::new(Node::Literal(1.0)),
                Box::new(Node::Literal(2.0)),
            ))
        );

        assert_eq!(
            easy_make_node(paren!("3", "+", "A2", "*", "5")),
            Ok(Node::BinaryOp(
                BinaryOp::Add,
                Box::new(Node::Literal(3.0)),
                Box::new(Node::BinaryOp(
                    BinaryOp::Multiply,
                    Box::new(Node::Reference("A2".to_string())),
                    Box::new(Node::Literal(5.0)),
                )),
            ))
        );

        assert_eq!(
            easy_make_node(paren!("3", "*", "4", "+", "5")),
            Ok(Node::BinaryOp(
                BinaryOp::Add,
                Box::new(Node::BinaryOp(
                    BinaryOp::Multiply,
                    Box::new(Node::Literal(3.0)),
                    Box::new(Node::Literal(4.0)),
                )),
                Box::new(Node::Literal(5.0)),
            ))
        );

        assert_eq!(
            easy_make_node(paren!(paren!("3", "+", "4"), "*", "CC100")),
            Ok(Node::BinaryOp(
                BinaryOp::Multiply,
                Box::new(Node::BinaryOp(
                    BinaryOp::Add,
                    Box::new(Node::Literal(3.0)),
                    Box::new(Node::Literal(4.0)),
                )),
                Box::new(Node::Reference("CC100".to_string())),
            ))
        );
    }
}
