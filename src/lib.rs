use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub fn parse_recursive_functions(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::functions, input)
}

pub fn parse_integer(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::integer, input)
}

pub fn parse_identifier(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::identifier, input)
}

pub fn parse_zero(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::zero, input)
}

pub fn parse_successor(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::successor, input)
}

pub fn parse_projection(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::projection, input)
}

pub fn parse_composition(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::composition, input)
}

pub fn parse_primitive(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::primitive, input)
}

pub fn parse_minimization(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::minimization, input)
}

pub fn parse_recursive_function(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Grammar::parse(Rule::primitive, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_test_1() {
        let got_res = parse_integer("10");
        assert!(got_res.is_ok());
        let pair = got_res.unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::integer);
        assert_eq!(pair.as_span().as_str(), "10");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);
        assert!(pair.into_inner().next().is_none());
    }
    
    #[test]
    fn integer_test_2() {
        assert!(parse_integer("  10  ").is_err());
        assert!(parse_integer("-10").is_err());
        assert!(parse_integer("abc").is_err());
        assert!(parse_integer("").is_err());
    }

}