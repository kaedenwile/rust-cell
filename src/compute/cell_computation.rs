#[derive(Clone)]
pub struct CellComputation {
    /// Has this value been computed yet or is it still pending?
    /// Can be true even when value is None
    pub is_computed: bool,
    pub error: bool,
    pub display: String,
    pub value: Option<f64>,
}

impl CellComputation {
    pub const fn blank() -> Self {
        CellComputation {
            is_computed: false,
            error: false,
            display: String::new(),
            value: None,
        }
    }

    pub fn string(string: String) -> Self {
        // Convert to float with best effort
        let value = string.parse::<f64>().ok();

        CellComputation {
            is_computed: true,
            error: false,
            display: string,
            value,
        }
    }

    pub fn error(err: String) -> Self {
        CellComputation {
            is_computed: true,
            error: true,
            display: err,
            value: None,
        }
    }

    pub fn value(value: f64) -> Self {
        CellComputation {
            is_computed: true,
            error: false,
            display: format!("{}", value),
            value: Some(value),
        }
    }
}

impl Default for CellComputation {
    fn default() -> Self {
        Self::blank()
    }
}