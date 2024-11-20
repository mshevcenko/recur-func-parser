# recur-func-parser

## General information

Repository:
 - https://github.com/mshevcenko/recur-func-parser.git
Crate:
 - 
Documentation:
 - 

## Overview

This parser is built for general/partial recursive function parsing.
After all functions are parsed, they can be used for calculations.
Anyone could practice creating a general recursive function using this parser.

For more information about general recursive function: 
 - https://en.wikipedia.org/wiki/General_recursive_function

zero function marking: "$z"  
successor function marking: "$s"  
projection function marking: "$p&lt;arguments count&gt;.&lt;argument number&gt;"  
composition function marking: "(&lt;func&gt; : &lt;func&gt;, ... , &lt;func&gt;)"  
primitive function marking: "[&lt;func&gt; , &lt;func&gt;]"  
minimization function marking: "{&lt;func&gt; , &lt;integer&gt;}"

## Parsing Process

The parsing process follows these steps:

1. **File Parsing:** The parser first reads the entire content of the any given text file which contains recursive functions with their identifiers.
2. **Function Parsing:** The parser parses recursive function into RecurFunction struct and its identifier. Then they are added into HashMap where key is identifier and value is recursive function.
3. **Execution:** The parser parses queries into Query struct which contains identifier of function and arguments for calculations. Then this query executes on parsed functions before and returns result, number if result is defined, otherwise undefined.

## Grammar

```pest
WHITESPACE = _{ " " | NEWLINE | "\t" }
integer = @{ ASCII_DIGIT+ }
identifier = @{ ASCII_ALPHA ~ (ASCII_DIGIT | ASCII_ALPHA)* }
zero = { "$z" }
successor = { "$s" }
projection = ${ "$p" ~ integer ~ "." ~ integer }
composition = { "(" ~ recursive_function ~ ":" ~ recursive_function ~ ("," ~ recursive_function)* ~ ")" }
primitive = { "[" ~ recursive_function ~ "," ~ recursive_function ~ "]" }
minimization = { "{" ~ recursive_function ~ "," ~ integer ~ "}" }
recursive_function = { zero | successor | projection | identifier | composition | primitive | minimization }
functions = { SOI ~ (identifier ~ "=" ~ recursive_function ~ ";")+ ~ EOI }
query = { SOI ~ identifier ~ integer* ~ EOI }
```

## Example

### CLI

```bash
# Show help information
recur-func-parser help

# Show credits information
recur-func-parser credits

# Parse recursive functions
recur-func-parser parse recur_functions.txt

# Parse recursive functions and print result
recur-func-parser parse recur_functions.txt -p

# Parse recursive functions and start execution loop
recur-func-parser parse recur_functions.txt -e
```

### CLI Execution loop example

```bash
recur-func-parser parse recur_functions.txt -e
Execution loop started. To stop it, type: ':exit'
> addition 4 5
Result: 9
> const2
Result: 2
> subtractionPart 5 7
Result: Undefined
```