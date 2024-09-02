use crate::compute::node::resolve_reference;
use crate::compute::parse::parse;
use crate::state::State;

// Populate the COMPUTED value for a State
pub fn bake(state: &mut State) {
    // Clear all cells
    for row in state.content.iter_mut() {
        for cell in row.iter_mut() {
            cell.computed.clear();
        }
    }

    fn parse_cell(state: &mut State, addr: (u16, u16), loop_stack: &Vec<(u16, u16)>) {
        if state.get_at(addr).computed.is_computed {
            return;
        }

        let mut cell = state.get_at(addr).clone();

        if cell.value.is_empty() {
            return;
        }

        let parse_result = parse(&cell.value.as_str());

        let Ok(node) = parse_result else {
            cell.computed.set_error(parse_result.unwrap_err());
            state.set_at(addr, cell);
            return;
        };

        // compute all references
        let mut new_loop_stack = loop_stack.clone();
        new_loop_stack.push(addr);

        for reference in &node.get_references() {
            let addr_result = resolve_reference(reference);

            let Ok(ref_addr) = addr_result else {
                cell.computed.set_error(addr_result.unwrap_err());
                state.set_at(addr, cell);
                return;
            };

            if loop_stack.contains(&ref_addr) {
                cell.computed
                    .set_error(format!("Data contains a cycle! {:?}", loop_stack));
                state.set_at(addr, cell);
                return;
            }

            parse_cell(state, ref_addr, &new_loop_stack);
        }

        match node.compute(state) {
            Ok(val) => cell.computed.set_computed(val),
            Err(err) => cell.computed.set_error(err),
        }

        state.set_at(addr, cell);
    }

    let num_rows = state.content.len();
    for r in 0..num_rows {
        let num_columns = state.content[r].len();
        for c in 0..num_columns {
            parse_cell(state, (r as u16, c as u16), &vec![]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::DisplayCell;

    const REF: fn(&str) -> (u16, u16) = |s: &str| resolve_reference(&s.to_string()).unwrap();

    #[test]
    fn test_bake() {
        let mut state = State::blank();

        state.set_at(REF("A1"), DisplayCell::new("4 * ( 2 + 3 )".to_string()));
        state.set_at(REF("B1"), DisplayCell::new("A1 * 2".to_string()));
        state.set_at(REF("C1"), DisplayCell::new("B1 * 2".to_string()));
        bake(&mut state);

        let a1 = state.get_at(REF("A1"));
        assert_eq!(a1.computed.is_computed, true);
        assert_eq!(a1.computed.error, false);
        assert_eq!(a1.computed.display, "20");
        assert_eq!(a1.computed.value, Some(20.0));

        let b1 = state.get_at(REF("B1"));
        assert_eq!(b1.computed.is_computed, true);
        assert_eq!(b1.computed.error, false);
        assert_eq!(b1.computed.display, "40");
        assert_eq!(b1.computed.value, Some(40.0));

        let c1 = state.get_at(REF("C1"));
        assert_eq!(c1.computed.is_computed, true);
        assert_eq!(c1.computed.error, false);
        assert_eq!(c1.computed.display, "80");
        assert_eq!(c1.computed.value, Some(80.0));
    }

    #[test]
    fn test_bake_2() {
        let mut state = State::blank();
        state.set_at(REF("C3"), DisplayCell::new("100".to_string()));
        state.set_at(REF("C4"), DisplayCell::new("C3 + 1".to_string()));
        state.set_at(REF("C5"), DisplayCell::new("C4 + 1".to_string()));
        bake(&mut state);

        let a1 = state.get_at(REF("C3"));
        assert_eq!(a1.computed.is_computed, true);
        assert_eq!(a1.computed.error, false);
        assert_eq!(a1.computed.display, "100");
        assert_eq!(a1.computed.value, Some(100.0));

        let b1 = state.get_at(REF("C4"));
        assert_eq!(b1.computed.is_computed, true);
        assert_eq!(b1.computed.error, false);
        assert_eq!(b1.computed.display, "101");
        assert_eq!(b1.computed.value, Some(101.0));

        let c1 = state.get_at(REF("C5"));
        assert_eq!(c1.computed.is_computed, true);
        assert_eq!(c1.computed.error, false);
        assert_eq!(c1.computed.display, "102");
        assert_eq!(c1.computed.value, Some(102.0));
    }

    #[test]
    fn test_bake_cycle() {
        let mut state = State::blank();
        state.set_at(REF("A1"), DisplayCell::new("5 + C3".to_string()));
        state.set_at(REF("B2"), DisplayCell::new("A1 - 4".to_string()));
        state.set_at(REF("C3"), DisplayCell::new("B2 * 10".to_string()));
        state.set_at(REF("D4"), DisplayCell::new("3.14".to_string()));
        bake(&mut state);

        let a1 = state.get_at(REF("A1"));
        assert_eq!(a1.computed.is_computed, true);
        assert_eq!(a1.computed.error, true);
        // assert_eq!(a1.computed.display, "100");
        assert_eq!(a1.computed.value, None);

        let b1 = state.get_at(REF("B2"));
        assert_eq!(b1.computed.is_computed, true);
        assert_eq!(b1.computed.error, true);
        // assert_eq!(b1.computed.display, "101");
        assert_eq!(b1.computed.value, None);

        let c1 = state.get_at(REF("C3"));
        assert_eq!(c1.computed.is_computed, true);
        assert_eq!(c1.computed.error, true);
        // assert_eq!(c1.computed.display, "102");
        assert_eq!(c1.computed.value, None);

        let c1 = state.get_at(REF("D4"));
        assert_eq!(c1.computed.is_computed, true);
        assert_eq!(c1.computed.error, false);
        assert_eq!(c1.computed.display, "3.14");
        assert_eq!(c1.computed.value, Some(3.14));
    }
}
