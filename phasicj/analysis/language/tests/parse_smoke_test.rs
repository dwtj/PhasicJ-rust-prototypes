
#[cfg(test)]
mod tests {
    #[test]
    pub fn main() {
        use phasicj_analysis_language::parse;
        assert!(parse("f(X).").is_ok());
    }
}