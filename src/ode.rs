// a solver for a single equation
pub struct ODE {
    right: String,
}

impl ODE {
    pub fn new(right: String) -> Self {
        Self { right }
    }

    // init
    fn parse() -> Result<(), String> {
        Err("TODO".to_string())
    }
}

impl Iterator for ODE {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
