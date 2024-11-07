use math_expr_parser::parse_recursive_functions;

fn main() -> anyhow::Result< () > {
    let got = parse_recursive_functions("addition = [p1.1, (s:p3.3)];")?;
    println!("{:?}", got);
    Ok(())
}
