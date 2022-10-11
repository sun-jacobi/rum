pub struct ODE {
    right: String,
}

impl ODE {
    pub fn new(right: String) -> Self {
        Self { right }
    }
}

impl Iterator for ODE {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
