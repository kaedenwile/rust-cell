use crate::compute::node::resolve_reference;
use crate::compute::parse::parse;
use crate::compute::CellComputation;
use crate::sheet::Grid;

// Populate the COMPUTED value for a State
// TODO: Optimize to recompute only dirty cells
pub fn bake(values: &Grid<String>, output: &mut Grid<CellComputation>) {
    // Clear all cells
    for addr in values.iter_addresses() {
        if output.get_at(addr).is_some() {
            output.set_at(addr, CellComputation::blank());
        }
    }

    fn parse_cell(values: &Grid<String>, output: &mut Grid<CellComputation>, addr: (u16, u16), loop_stack: &Vec<(u16, u16)>) {
        if output.get_at(addr).is_some_and(|output| output.is_computed) {
            return;
        }
        let Some(value) = values.get_at(addr) else {
            output.set_at(addr, CellComputation::blank()); // TODO is this needed?
            return;
        };

        // Only parse if it starts with '='
        if !value.starts_with('=') {
            let text = value.to_string();
            output.set_at(addr, CellComputation::string(text));
            return;
        }

        let formula = &value[1..].to_string();
        let parse_result = parse(formula);

        let Ok(node) = parse_result else {
            output.set_at(addr, CellComputation::error(parse_result.unwrap_err()));
            return;
        };

        // compute all references
        let mut new_loop_stack = loop_stack.clone();
        new_loop_stack.push(addr);

        for reference in &node.get_references() {
            let addr_result = resolve_reference(reference);

            let Ok(ref_addr) = addr_result else {
                output.set_at(addr, CellComputation::error(addr_result.unwrap_err()));
                return;
            };

            if loop_stack.contains(&ref_addr) {
                output.set_at(addr, CellComputation::error(format!("Data contains a cycle! {:?}", loop_stack)));
                return;
            }

            parse_cell(values, output, ref_addr, &new_loop_stack);
        }

        output.set_at(addr, match node.compute(output) {
            Ok(val) => CellComputation::value(val),
            Err(err) => CellComputation::error(err),
        })
    }

    for addr in values.iter_addresses() {
        parse_cell(values, output, addr, &vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REF: fn(&str) -> (u16, u16) = |s: &str| resolve_reference(&s.to_string()).unwrap();

    #[test]
    fn test_bake() {
        let mut values = Grid::new();
        let mut output = Grid::new();

        values.set_at(REF("A1"), "4 * ( 2 + 3 )".to_string());
        values.set_at(REF("B1"), "A1 * 2".to_string());
        values.set_at(REF("C1"), "B1 * 2".to_string());
        bake(&values, &mut output);

        let a1 = output.get_at(REF("A1")).unwrap();
        assert_eq!(a1.is_computed, true);
        assert_eq!(a1.error, false);
        assert_eq!(a1.display, "20");
        assert_eq!(a1.value, Some(20.0));

        let b1 = output.get_at(REF("B1")).unwrap();
        assert_eq!(b1.is_computed, true);
        assert_eq!(b1.error, false);
        assert_eq!(b1.display, "40");
        assert_eq!(b1.value, Some(40.0));

        let c1 = output.get_at(REF("C1")).unwrap();
        assert_eq!(c1.is_computed, true);
        assert_eq!(c1.error, false);
        assert_eq!(c1.display, "80");
        assert_eq!(c1.value, Some(80.0));
    }

    #[test]
    fn test_bake_2() {
        let mut values = Grid::new();
        let mut output = Grid::new();

        values.set_at(REF("C3"), "100".to_string());
        values.set_at(REF("C4"), "C3 + 1".to_string());
        values.set_at(REF("C5"), "C4 + 1".to_string());
        bake(&values, &mut output);

        let a1 = output.get_at(REF("C3")).unwrap();
        assert_eq!(a1.is_computed, true);
        assert_eq!(a1.error, false);
        assert_eq!(a1.display, "100");
        assert_eq!(a1.value, Some(100.0));

        let b1 = output.get_at(REF("C4")).unwrap();
        assert_eq!(b1.is_computed, true);
        assert_eq!(b1.error, false);
        assert_eq!(b1.display, "101");
        assert_eq!(b1.value, Some(101.0));

        let c1 = output.get_at(REF("C5")).unwrap();
        assert_eq!(c1.is_computed, true);
        assert_eq!(c1.error, false);
        assert_eq!(c1.display, "102");
        assert_eq!(c1.value, Some(102.0));
    }

    #[test]
    fn test_bake_cycle() {
        let mut values = Grid::new();
        let mut output = Grid::new();

        values.set_at(REF("A1"), "5 + C3".to_string());
        values.set_at(REF("B2"), "A1 - 4".to_string());
        values.set_at(REF("C3"), "B2 * 10".to_string());
        values.set_at(REF("D4"), "3.14".to_string());
        bake(&values, &mut output);

        let a1 = output.get_at(REF("A1")).unwrap();
        assert_eq!(a1.is_computed, true);
        assert_eq!(a1.error, true);
        // assert_eq!(a1.display, "100");
        assert_eq!(a1.value, None);

        let b1 = output.get_at(REF("B2")).unwrap();
        assert_eq!(b1.is_computed, true);
        assert_eq!(b1.error, true);
        // assert_eq!(b1.display, "101");
        assert_eq!(b1.value, None);

        let c1 = output.get_at(REF("C3")).unwrap();
        assert_eq!(c1.is_computed, true);
        assert_eq!(c1.error, true);
        // assert_eq!(c1.display, "102");
        assert_eq!(c1.value, None);

        let d4 = output.get_at(REF("D4")).unwrap();
        assert_eq!(d4.is_computed, true);
        assert_eq!(d4.error, false);
        assert_eq!(d4.display, "3.14");
        assert_eq!(d4.value, Some(3.14));
    }
}
