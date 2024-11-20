use recur_func_parser::{execute_query, parse_query, parse_recur_functions, RecurFunction};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

const HELP: &str = r#"
Usage: recur-func-parser <COMMAND> [OPTIONS]

Commands:
  parse <FILE_PATH>         Parse file containing general recursive functions
    Options:
      -p, --print           Print parsing result
      -e, --execute         Start execution loop to execute input queries. To stop it, type: ':exit'

  help                      Print this help message
  credits                   Print project credits and information
"#;

const CREDITS: &str = r#"
recur-func-parser v1.0

Author:
  Mykhailo Shevchenko <mh.shevchenko@ukma.edu.ua>

Repository:
  https://github.com/mshevcenko/recur-func-parser.git

Built with:
  - Rust Programming Language
  - pest (Parser Generator)

License: MIT
"#;

fn execution_loop(identifier_functions: &HashMap<String, RecurFunction>) {
    println!("Execution loop started. To stop it, type: ':exit'");
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        if let Some(':') = input_line.chars().next() {
            break;
        }
        let query_res = parse_query(&input_line, identifier_functions);
        match query_res {
            Ok(query) => match execute_query(&query, identifier_functions) {
                Some(number) => println!("Result: {number}"),
                None => println!("Result: Undefined"),
            },
            Err(error) => {
                eprintln! {"Error: {}", error};
            }
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{HELP}");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "help" => {
            println!("{HELP}");
        }
        "credits" => {
            println!("{CREDITS}");
        }
        "parse" => {
            if args.len() < 3 {
                eprintln!(
                    "No input file provided for parse command. Use 'help' for usage information"
                );
                return;
            }
            let file_content = std::fs::read_to_string(&args[2]).expect("could not read file");
            let identifier_functions = match parse_recur_functions(&file_content) {
                Ok(identifier_functions) => identifier_functions,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    return;
                }
            };
            let mut to_print: bool = false;
            let mut to_execute: bool = false;
            for arg in args.iter().skip(3) {
                match arg.as_str() {
                    "-p" | "--print" => {
                        to_print = true;
                    }
                    "-e" | "--execute" => {
                        to_execute = true;
                    }
                    _ => {
                        eprintln!("Unknown option: {}. Use 'help' for usage information", arg);
                        return;
                    }
                }
            }
            if to_print {
                println!("{:?}", identifier_functions);
            }
            if to_execute {
                execution_loop(&identifier_functions);
            }
        }
        _ => {
            eprintln!("Unknown command: {command}. Use 'help' for usage information");
        }
    }
}
