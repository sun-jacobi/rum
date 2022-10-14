pub mod ode;
mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn ode_simple_test() {
        assert!(1 + 1 == 2);
    }
}
