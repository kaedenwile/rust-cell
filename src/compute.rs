use crate::state::State;

pub fn compute(state: &mut State) {
    for row in state.content.iter_mut() {
        for cell in row.iter_mut() {
            if cell.value.is_empty() {
                continue;
            }

            let node = parse(&cell.value.as_str());
            match node {
                Ok(node) => {
                    cell.error = false;
                    cell.computed = format!("{:?}", node);
                }
                Err(e) => {
                    cell.error = true;
                    cell.computed = format!("{}", e);
                }
            }
        }
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
    if !current_term.is_empty() {
        terms.push(current_term);
    }

    terms
}

#[derive(Debug)]
enum ParenStack {
    Term(String),
    Parens(Vec<ParenStack>),
}
fn reduce_paren_stack(terms: Vec<String>) -> Result<ParenStack, String> {
    let mut root = ParenStack::Parens(Vec::new());
    let mut depth = 0;

    fn push(root: &mut ParenStack, depth: i32, term: ParenStack) {
        print!("TERM={:?}", term);

        let mut target = root;
        for i in 0..depth {
            print!("\tSTEP 123 i={} depth={} {:?}", i, depth, target);

            match target {
                ParenStack::Parens(ref mut vec) => {
                    target = vec.last_mut().unwrap();
                }
                _ => panic!("Internal err!"),
            }
        }

        println!("\tPUSHING {:?}", target);
        match target {
            ParenStack::Parens(ref mut vec) => vec.push(term),
            _ => panic!("Internal err!"),
        }
    }
    ;

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

    // drop(terms);
    Ok(root)
}

// fn make_node<'a>(raw_terms: Vec<ParenStack>) -> Result<&'a Node<'a>, String> {
//     enum Computed<'a> {
//         Computed(&'a Node<'a>),
//         Raw(&'a String),
//     }
//
//     // Reduce parentheses and parse numbers
//     let mut terms: Vec<Computed> = Vec::new();
//     for x in raw_terms.into_iter() {
//         terms.push(match x {
//             ParenStack::Term(term) => match term.parse::<f32>() {
//                 Ok(f) => Computed::Computed(&Node::Literal(f)),
//                 Err(_) => Computed::Raw(term),
//             },
//             ParenStack::Parens(terms) => Computed::Computed(match make_node(terms) {
//                 Ok(node) => node,
//                 Err(e) => return Err(e),
//             }),
//         })
//     }
//
//     // Reduce multiplication and division
//     let mut cursor: usize = 0;
//     while cursor < terms.len() {
//         let op = match terms[cursor] {
//             Computed::Raw(op) if op == "*" || op == "/" => op,
//             _ => {
//                 cursor += 1;
//                 continue;
//             }
//         };
//
//         if cursor < 1 || cursor >= terms.len() - 1 {
//             return Err("Multiplication or division at boundary".to_string());
//         }
//
//         let prev = match terms.remove(cursor - 1) {
//             Computed::Raw(_) => return Err("Bad multiplication after raw".to_string()),
//             Computed::Computed(node) => node,
//         };
//         let next = match terms.remove(cursor) {
//             Computed::Raw(_) => return Err("Bad multiplication before raw".to_string()),
//             Computed::Computed(node) => node,
//         };
//
//         terms[cursor - 1] = Computed::Computed(&Node::BinaryOp(
//             match op.as_str() {
//                 "*" => BinaryOp::Multiply,
//                 "/" => BinaryOp::Divide,
//                 _ => panic!("Internal err"),
//             },
//             prev,
//             next,
//         ));
//     }
//
//     // Reduce addition and subtraction
//     cursor = 0;
//     while cursor < terms.len() {
//         let op = match terms[cursor] {
//             Computed::Raw(op) if op == "+" || op == "-" => op,
//             _ => {
//                 cursor += 1;
//                 continue;
//             }
//         };
//
//         if cursor < 1 || cursor >= terms.len() - 1 {
//             return Err("Addition or Subtraction at boundary".to_string());
//         }
//
//         let prev = match terms.remove(cursor - 1) {
//             Computed::Raw(_) => return Err("Bad addition after raw".to_string()),
//             Computed::Computed(node) => node,
//         };
//         let next = match terms.remove(cursor) {
//             Computed::Raw(_) => return Err("Bad addition before raw".to_string()),
//             Computed::Computed(node) => node,
//         };
//
//         terms[cursor - 1] = Computed::Computed(&Node::BinaryOp(
//             match op.as_str() {
//                 "+" => BinaryOp::Add,
//                 "-" => BinaryOp::Subtract,
//                 _ => panic!("Internal err"),
//             },
//             &prev,
//             &next,
//         ));
//     }
//
//     if terms.len() != 1 {
//         Err("Could not fully reduce".to_string())
//     } else {
//         match &terms[0] {
//             Computed::Raw(_) => Err("Could not reduce from raw".to_string()),
//             Computed::Computed(node) => Ok(node),
//         }
//     }
//
//     // let mut current_term = String::from("");
//     // let mut root: Some<Node> = None;
//     //
//     // for (i, char) in cell.chars().enumerate() {
//     //     match char {
//     //         // UNARY OPS
//     //         '-' if current_term.is_empty() => {
//     //             return match parse(&cell[i..]) {
//     //                 Ok(x) => Ok(Node::UnaryOp(UnaryOp::Negative, x)),
//     //                 Err(e) => Err(e),
//     //             }
//     //         }
//     //         // BINARY OPS
//     //         '+' | '-' | '*' | '/' => {
//     //             let float = match current_term.parse::<f32>() {
//     //                 Ok(x) => x,
//     //                 Err(e) => return Err(e),
//     //             };
//     //
//     //             let op = match char {
//     //                 '+' => BinaryOp::Add,
//     //                 '-' => BinaryOp::Subtract,
//     //                 '*' => BinaryOp::Multiply,
//     //                 '/' => BinaryOp::Divide,
//     //                 _ => panic!("Bad binary operator"),
//     //             };
//     //
//     //             return match parse(&cell[i..]) {
//     //                 Ok(x) => Ok(Node::BinaryOp(op, Node::Literal(float), x)),
//     //                 Err(e) => Err(e),
//     //             };
//     //         }
//     //         '(' => {
//     //             if current_term == "SUM" || current_term == "AVG" {
//     //                 // parse function args
//     //             } else {
//     //                 // parens
//     //             }
//     //         }
//     //
//     //         // ignore whitespace
//     //         ' ' => {}
//     //
//     //         //
//     //         _ => current_term.push(char),
//     //     }
//     // }
// }

// fn parse(cell: &str) -> Result<&Node, String> {
fn parse(cell: &str) -> Result<ParenStack, String> {
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

    Ok(paren_stack)
    //
    // let mut terms = match paren_stack {
    //     ParenStack::Parens(terms) => terms,
    //     _ => panic!("Internal err"),
    // };
    // make_node(terms)
}

#[derive(Debug)]
enum Node<'a> {
    Literal(f32),
    // UnaryOp(UnaryOp, Node),
    BinaryOp(BinaryOp, &'a Node<'a>, &'a Node<'a>),
    // Function(Function, Vec<Node>),
    // Reference(String),
}

enum UnaryOp {
    Negative,
}

#[derive(Debug)]
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Function {
    Sum,
    Avg,
    Pow,
}
