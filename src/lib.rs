use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;


#[derive(Parser)]
#[grammar = "./grammar.pest"]
/// The grammar parser for recusrive functions based on Pest grammar definition.
pub struct RecurFunctionGrammar;

#[derive(Debug, Error)]
/// Errors which can apear when parsing.
pub enum RecurFunctionParseError {
    #[error("Invalid projection argument number: {0}")]
    /// Error which signals that argument number in projection function is invalid.
    InvalidProjectionArgumentNumber(String),
    #[error("Invalid composition functions count: {0}")]
    /// Error which signals about invalid number of functions in composition function.
    InvalidCompositionFunctionsCount(String),
    #[error("Invalid primitive base arguments count: {0}")]
    /// Error which signals that base function in primitive function has invalid arguments count.
    InvalidPrimitiveBaseArgumentsCount(String),
    #[error("Invalid primitive step arguments count: {0}")]
    /// Error which signals that step function in primitive function has invalid arguments count.
    InvalidPrimitiveStepArgumentsCount(String),
    #[error("Invalid arguments count: {0}")]
    /// Error which signals that function has invalid arguments count.
    InvalidArgumentsCount(String),
    #[error("Expected function, but it wasn't there: {0}")]
    /// Error which signals that function was expected, but not found.
    FunctionExpected(String),
    #[error("Expected integer, but it wasn't there: {0}")]
    /// Error which signals that integer was expected, but not found.
    IntegerExpected(String),
    #[error("Expected identifier, but it wasn't there: {0}")]
    /// Error which signals that identifier was expected, but not found.
    IdentifierExpected(String),
    #[error("Failed to parse int: {0}")]
    /// Error which signals that string cannot be converted to u32.
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Undefined identifier while parsing: {0}")]
    /// Error which signals that identifier is undefined.
    UndefinedIdentifier(String),
    #[error("Identifier already exists: {0}")]
    /// Error which signals that identifier already exists.
    IdentifierAlreadyExists(String),
    #[error("Undefined rule while parsing: {0}")]
    /// Error which signals that rule is undefined.
    UndefinedRule(String),
}

#[derive(Debug, Clone, PartialEq)]
/// Types of recursive functions.
pub enum RecurFunctionType {
    /// zero function.
    Zero,
    /// succesor function.
    Successor,
    /// projection function with arguments count and argument number.
    Projection(u32, u32),
    /// composition function with base function and functions to use.
    Composition(Box<RecurFunction>, Vec<RecurFunction>),
    /// primitive function with base function and step function.
    Primitive(Box<RecurFunction>, Box<RecurFunction>),
    /// minimization function with base function and max tries.
    Minimization(Box<RecurFunction>, u32),
}

#[derive(Debug, Clone, PartialEq)]
/// Struct which describes recursive function.
pub struct RecurFunction {
    /// function type.
    function_type: RecurFunctionType,
    /// function arguments count.
    arguments_count: u32,
    /// contains number if function is number otherwise None.
    number: Option<u32>,
}

/// Struct which describes query.
pub struct Query {
    /// identifier of function to use for query.
    identifier: String,
    /// arguments for function.
    arguments: Vec<u32>,
}

/// Parses recursive function pair into RecurFunction struct.
///
/// # Arguments
///
/// * `pair` - pest pair that is recursive function.
/// * `identifier_functions` - parsed identifiers and their function, uses for checking existing functions.
///
/// # Returns
///
/// The parsed recursive function or RecurFunctionParseError wraped in Result.
pub fn parse_recur_function(
    pair: pest::iterators::Pair<Rule>,
    identifier_functions: &HashMap<String, RecurFunction>,
) -> Result<RecurFunction, RecurFunctionParseError> {
    let pair_str = pair.as_str();
    match pair.as_rule() {
        Rule::zero => Ok(RecurFunction {
            function_type: RecurFunctionType::Zero,
            arguments_count: 1,
            number: Some(0),
        }),
        Rule::successor => Ok(RecurFunction {
            function_type: RecurFunctionType::Successor,
            arguments_count: 1,
            number: None,
        }),
        Rule::projection => {
            let mut inner_pairs = pair.into_inner();
            let arguments_count: u32 = inner_pairs
                .next()
                .ok_or(RecurFunctionParseError::IntegerExpected(
                    pair_str.to_string(),
                ))?
                .as_str()
                .parse::<u32>()?;
            let argument_number: u32 = inner_pairs
                .next()
                .ok_or(RecurFunctionParseError::IntegerExpected(
                    pair_str.to_string(),
                ))?
                .as_str()
                .parse::<u32>()?;
            if argument_number == 0 {
                return Err(RecurFunctionParseError::InvalidProjectionArgumentNumber(
                    pair_str.to_string(),
                ));
            }
            if arguments_count < argument_number {
                return Err(RecurFunctionParseError::InvalidArgumentsCount(
                    pair_str.to_string(),
                ));
            }
            Ok(RecurFunction {
                function_type: RecurFunctionType::Projection(arguments_count, argument_number),
                arguments_count,
                number: None,
            })
        }
        Rule::composition => {
            let mut inner_pairs = pair.into_inner();
            let base_function = parse_recur_function(
                inner_pairs
                    .next()
                    .ok_or(RecurFunctionParseError::FunctionExpected(
                        pair_str.to_string(),
                    ))?,
                identifier_functions,
            )?;
            let mut functions: Vec<RecurFunction> = Vec::new();
            let mut arguments_count: u32 = 0;
            for inner_pair in inner_pairs {
                let function = parse_recur_function(inner_pair, identifier_functions)?;
                if arguments_count == 0 {
                    arguments_count = function.arguments_count;
                } else if arguments_count != function.arguments_count {
                    return Err(RecurFunctionParseError::InvalidArgumentsCount(
                        pair_str.to_string(),
                    ));
                }
                functions.push(function);
            }
            if functions.len() != base_function.arguments_count as usize {
                return Err(RecurFunctionParseError::InvalidCompositionFunctionsCount(
                    pair_str.to_string(),
                ));
            }
            if arguments_count == 1
                && functions.len() == 1
                && functions[0].number.is_some()
                && base_function.function_type == RecurFunctionType::Successor
            {
                let number = functions[0].number.unwrap() + 1;
                return Ok(RecurFunction {
                    function_type: RecurFunctionType::Composition(
                        Box::new(base_function),
                        functions,
                    ),
                    arguments_count,
                    number: Some(number),
                });
            }
            Ok(RecurFunction {
                function_type: RecurFunctionType::Composition(Box::new(base_function), functions),
                arguments_count,
                number: None,
            })
        }
        Rule::primitive => {
            let mut inner_pairs = pair.into_inner();
            let base_function = parse_recur_function(
                inner_pairs
                    .next()
                    .ok_or(RecurFunctionParseError::FunctionExpected(
                        pair_str.to_string(),
                    ))?,
                identifier_functions,
            )?;
            let step_function = parse_recur_function(
                inner_pairs
                    .next()
                    .ok_or(RecurFunctionParseError::FunctionExpected(
                        pair_str.to_string(),
                    ))?,
                identifier_functions,
            )?;
            if step_function.arguments_count < 2 {
                return Err(RecurFunctionParseError::InvalidPrimitiveStepArgumentsCount(
                    pair_str.to_string(),
                ));
            }
            if step_function.arguments_count == 2
                && (base_function.arguments_count != 1 || base_function.number.is_none())
            {
                return Err(RecurFunctionParseError::InvalidPrimitiveBaseArgumentsCount(
                    pair_str.to_string(),
                ));
            }
            if step_function.arguments_count > 2
                && base_function.arguments_count != step_function.arguments_count - 2
            {
                return Err(RecurFunctionParseError::InvalidPrimitiveBaseArgumentsCount(
                    pair_str.to_string(),
                ));
            }
            let arguments_count = step_function.arguments_count - 1;
            Ok(RecurFunction {
                function_type: RecurFunctionType::Primitive(
                    Box::new(base_function),
                    Box::new(step_function),
                ),
                arguments_count,
                number: None,
            })
        }
        Rule::minimization => {
            let mut inner_pairs = pair.into_inner();
            let base_function = parse_recur_function(
                inner_pairs
                    .next()
                    .ok_or(RecurFunctionParseError::FunctionExpected(
                        pair_str.to_string(),
                    ))?,
                identifier_functions,
            )?;
            let max: u32 = inner_pairs
                .next()
                .ok_or(RecurFunctionParseError::IntegerExpected(
                    pair_str.to_string(),
                ))?
                .as_str()
                .parse::<u32>()?;
            if base_function.arguments_count <= 1 {
                return Err(RecurFunctionParseError::InvalidArgumentsCount(
                    pair_str.to_string(),
                ));
            }
            let arguments_count = base_function.arguments_count - 1;
            Ok(RecurFunction {
                function_type: RecurFunctionType::Minimization(Box::new(base_function), max),
                arguments_count,
                number: None,
            })
        }
        Rule::identifier => {
            let identifier: String = pair.as_str().to_string();
            let function = identifier_functions
                .get(&identifier)
                .ok_or(RecurFunctionParseError::UndefinedIdentifier(
                    pair.as_str().to_string(),
                ))?
                .clone();
            Ok(function)
        }
        Rule::recursive_function => parse_recur_function(
            pair.into_inner()
                .next()
                .ok_or(RecurFunctionParseError::FunctionExpected(
                    pair_str.to_string(),
                ))?,
            identifier_functions,
        ),
        _ => Err(RecurFunctionParseError::UndefinedRule(
            pair.as_str().to_string(),
        )),
    }
}

/// Parses recursive functions input into HashMap<String, RecurFunction> where key is identifier and value is its recursive function.
///
/// # Arguments
///
/// * `input` - string which includes recursive functions.
///
/// # Returns
///
/// HashMap<String, RecurFunction> where key is identifier and value is its recursive function or RecurFunctionParseError wraped into Result.
pub fn parse_recur_functions(
    input: &str,
) -> Result<HashMap<String, RecurFunction>, RecurFunctionParseError> {
    let got = RecurFunctionGrammar::parse(Rule::functions, input);
    let mut inner_pairs = match got {
        Ok(mut got) => got
            .next()
            .ok_or(RecurFunctionParseError::UndefinedRule(input.to_string()))?
            .into_inner(),
        Err(e) => return Err(RecurFunctionParseError::UndefinedRule(e.to_string())),
    };
    let mut identifier_functions = HashMap::<String, RecurFunction>::new();
    while let Some(inner_pair) = inner_pairs.next() {
        if inner_pair.as_rule() == Rule::EOI {
            break;
        }
        let identifier: String = match inner_pair.as_rule() {
            Rule::identifier => inner_pair.as_str().to_string(),
            _ => {
                return Err(RecurFunctionParseError::IdentifierExpected(
                    inner_pair.as_str().to_string(),
                ))
            }
        };
        if identifier_functions.contains_key(&identifier) {
            return Err(RecurFunctionParseError::IdentifierAlreadyExists(identifier));
        }
        let inner_pair = inner_pairs
            .next()
            .ok_or(RecurFunctionParseError::FunctionExpected(input.to_string()))?;
        let recur_function: RecurFunction = match inner_pair.as_rule() {
            Rule::recursive_function => parse_recur_function(inner_pair, &identifier_functions)?,
            _ => {
                return Err(RecurFunctionParseError::FunctionExpected(
                    inner_pair.as_str().to_string(),
                ))
            }
        };
        identifier_functions.insert(identifier, recur_function);
    }
    Ok(identifier_functions)
}

/// Parses query input string into Query struct.
///
/// # Arguments
///
/// * `input` - string which includes query.
/// * `identifier_functions` - parsed identifiers and their function, uses for checking existing functions.
///
/// # Returns
///
/// parsed query or RecurFunctionParseError wraped into Result.
pub fn parse_query(
    input: &str,
    identifier_functions: &HashMap<String, RecurFunction>,
) -> Result<Query, RecurFunctionParseError> {
    let got = RecurFunctionGrammar::parse(Rule::query, input);
    let mut inner_pairs = match got {
        Ok(mut got) => got
            .next()
            .ok_or(RecurFunctionParseError::UndefinedRule(input.to_string()))?
            .into_inner(),
        Err(_) => return Err(RecurFunctionParseError::UndefinedRule(input.to_string())),
    };
    let inner_pair = inner_pairs
        .next()
        .ok_or(RecurFunctionParseError::IdentifierExpected(
            input.to_string(),
        ))?;
    let identifier: String = match inner_pair.as_rule() {
        Rule::identifier => inner_pair.as_str().to_string(),
        _ => {
            return Err(RecurFunctionParseError::IdentifierExpected(
                inner_pair.as_str().to_string(),
            ))
        }
    };
    if !identifier_functions.contains_key(&identifier) {
        return Err(RecurFunctionParseError::UndefinedIdentifier(
            input.to_string(),
        ));
    }
    let function = identifier_functions.get(&identifier).ok_or(
        RecurFunctionParseError::UndefinedIdentifier(input.to_string()),
    )?;
    let mut arguments = Vec::<u32>::new();
    for inner_pair in inner_pairs {
        if inner_pair.as_rule() == Rule::EOI {
            break;
        }
        let integer: u32 = match inner_pair.as_rule() {
            Rule::integer => inner_pair.as_str().parse::<u32>()?,
            _ => {
                return Err(RecurFunctionParseError::IntegerExpected(
                    inner_pair.as_str().to_string(),
                ))
            }
        };
        arguments.push(integer);
    }
    if function.number.is_some() && arguments.is_empty() {
        return Ok(Query {
            identifier,
            arguments,
        });
    } else if function.arguments_count != arguments.len() as u32 {
        return Err(RecurFunctionParseError::InvalidArgumentsCount(
            input.to_string(),
        ));
    }
    Ok(Query {
        identifier,
        arguments,
    })
}

/// Parses given recursive functions on given arguments.
///
/// # Arguments
///
/// * `function` - function to execute.
/// * `arguments` - arguments to use for calculations.
///
/// # Returns
///
/// Some(u32) if result is defined otherwise None.
pub fn execute(function: &RecurFunction, arguments: &Vec<u32>) -> Option<u32> {
    match &function.function_type {
        RecurFunctionType::Zero => Some(0),
        RecurFunctionType::Successor => Some(arguments.first()? + 1),
        RecurFunctionType::Projection(_, argument_number) => {
            let res = arguments.get(*argument_number as usize - 1)?;
            Some(*res)
        }
        RecurFunctionType::Composition(base_function, functions) => {
            let mut functions_results: Vec<u32> = Vec::<u32>::new();
            for func in functions {
                functions_results.push(execute(func, arguments)?);
            }
            execute(base_function, &functions_results)
        }
        RecurFunctionType::Primitive(base_function, step_function) => {
            let mut res: u32;
            let mut arguments = arguments.clone();
            let max: u32 = arguments.pop()?;
            if let Some(number) = base_function.number {
                res = number;
            } else {
                res = execute(base_function, &arguments)?;
            }
            for i in 0..max {
                let mut new_arguments = arguments.clone();
                new_arguments.push(i);
                new_arguments.push(res);
                res = execute(step_function, &new_arguments)?;
            }
            Some(res)
        }
        RecurFunctionType::Minimization(base_function, max) => {
            for i in 0..=*max {
                let mut new_arguments = arguments.clone();
                new_arguments.push(i);
                if execute(base_function, &new_arguments)? == 0 {
                    return Some(i);
                }
            }
            None
        }
    }
}

/// Parses given query on given possible functions.
///
/// # Arguments
///
/// * `query` - function to execute.
/// * `identifier_functions` - parsed identifiers and their function, uses for checking existing functions.
///
/// # Returns
///
/// Some(u32) if result is defined otherwise None.
pub fn execute_query(
    query: &Query,
    identifier_functions: &HashMap<String, RecurFunction>,
) -> Option<u32> {
    let function: &RecurFunction = identifier_functions.get(&query.identifier)?;
    if let Some(number) = function.number {
        return Some(number);
    }
    execute(function, &query.arguments)
}
