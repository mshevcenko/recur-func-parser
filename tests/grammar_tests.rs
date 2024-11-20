use pest::Parser;
use recur_func_parser::*;

mod grammar_tests {
    use super::*;

    #[test]
    fn whitespace_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::WHITESPACE, " ");
        assert!(pair_res.is_ok());

        let pair_res = RecurFunctionGrammar::parse(Rule::WHITESPACE, "\t");
        assert!(pair_res.is_ok());

        let pair_res = RecurFunctionGrammar::parse(Rule::WHITESPACE, "\t");
        assert!(pair_res.is_ok());

        let pair_res = RecurFunctionGrammar::parse(Rule::WHITESPACE, "m");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn integer_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::integer, "0");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "0");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);

        let pair_res = RecurFunctionGrammar::parse(Rule::integer, "123");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "123");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 3);

        let pair_res = RecurFunctionGrammar::parse(Rule::integer, "   123   ");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::integer, "abc");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn identifier_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::identifier, "a");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "a");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);

        let pair_res = RecurFunctionGrammar::parse(Rule::identifier, "r2D2");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "r2D2");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 4);

        let pair_res = RecurFunctionGrammar::parse(Rule::identifier, "   abc   ");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::identifier, "1bc");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn zero_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::zero, "$z");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "$z");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);

        let pair_res = RecurFunctionGrammar::parse(Rule::zero, "   $z   ");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::zero, "z");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn successor_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::successor, "$s");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "$s");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);

        let pair_res = RecurFunctionGrammar::parse(Rule::successor, "   $s   ");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::successor, "s");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn projection_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "$p1.1");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "$p1.1");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 5);

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "$ p 1 . 1 ");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "   $p1.1   ");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "$p");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "$p1");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "$p1.");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "p1.1");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn composition_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "($s:$p3.3)");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "($s:$p3.3)");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 10);

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "(addition:$p3.3,$p3.1)");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "(addition:$p3.3,$p3.1)");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 22);

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "( $s : $p3.3 )");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "( $s : $p3.3 )");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 14);

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "(s:)");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "(:p3.3)");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "s:p3.3");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "(sp3.3)");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::composition, "a");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn primitive_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[$z,(addition:$p3.3,$p3.1)]");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "[$z,(addition:$p3.3,$p3.1)]");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 27);

        let pair_res =
            RecurFunctionGrammar::parse(Rule::primitive, "[ $z , ( addition : $p3.3 , $p3.1 ) ]");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "[ $z , ( addition : $p3.3 , $p3.1 ) ]");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 37);

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[$p1.1,($s:$p3.3)]");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "[$p1.1,($s:$p3.3)]");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 18);

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[1,($s:$p3.3)]");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[$p1.1,1]");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[,($s:$p3.3)]");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[$p1.1,]");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "[$p1.1 ($s:$p3.3)]");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "$p1.1,($s:$p3.3)");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::primitive, "a");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn minimization_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{subtractionAbs3,100}");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "{subtractionAbs3,100}");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 21);

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{ subtractionAbs3 , 100 }");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "{ subtractionAbs3 , 100 }");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 25);

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{subtractionAbs3,$s}");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{100,100}");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{subtractionAbs3 100}");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{subtractionAbs3,}");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "{,100}");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "subtractionAbs3,100");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::minimization, "");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn recursive_function_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "$z");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "$z");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);
        assert_eq!(pair.into_inner().next().unwrap().as_rule(), Rule::zero);

        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "$s");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "$s");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);
        assert_eq!(pair.into_inner().next().unwrap().as_rule(), Rule::successor);

        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "$p1.1");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "$p1.1");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 5);
        assert_eq!(
            pair.into_inner().next().unwrap().as_rule(),
            Rule::projection
        );

        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "($s:$p3.3)");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "($s:$p3.3)");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 10);
        assert_eq!(
            pair.into_inner().next().unwrap().as_rule(),
            Rule::composition
        );

        let pair_res =
            RecurFunctionGrammar::parse(Rule::recursive_function, "[$z,(addition:$p3.3,$p3.1)]");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "[$z,(addition:$p3.3,$p3.1)]");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 27);
        assert_eq!(pair.into_inner().next().unwrap().as_rule(), Rule::primitive);

        let pair_res =
            RecurFunctionGrammar::parse(Rule::recursive_function, "{subtractionAbs3,100}");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "{subtractionAbs3,100}");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 21);
        assert_eq!(
            pair.into_inner().next().unwrap().as_rule(),
            Rule::minimization
        );

        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "addition");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "addition");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 8);
        assert_eq!(
            pair.into_inner().next().unwrap().as_rule(),
            Rule::identifier
        );

        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "100");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::recursive_function, "");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn functions_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "const0=$z;");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "const0=$z;");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 10);

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "const0 = $z ; ");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "const0 = $z ; ");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 14);

        let pair_res =
            RecurFunctionGrammar::parse(Rule::functions, "const0=$z;addition=[$p1.1,($s:$p3.3)];");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "const0=$z;addition=[$p1.1,($s:$p3.3)];");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 38);

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "const0=100;");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "const0=$z");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "const0 $z;");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "const0=;");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "=$z;");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::functions, "");
        assert!(pair_res.is_err());

        Ok(())
    }

    #[test]
    fn query_test() -> anyhow::Result<()> {
        let pair_res = RecurFunctionGrammar::parse(Rule::query, "addition");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "addition");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 8);
        let mut inner_pairs = pair.into_inner();
        assert_eq!(inner_pairs.next().unwrap().as_rule(), Rule::identifier);
        assert_eq!(inner_pairs.next().unwrap().as_rule(), Rule::EOI);

        let pair_res = RecurFunctionGrammar::parse(Rule::query, "addition 6 9");
        assert!(pair_res.is_ok());
        let pair = pair_res?
            .next()
            .ok_or_else(|| anyhow::anyhow!("pair expected"))?;
        assert_eq!(pair.as_str(), "addition 6 9");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 12);
        let mut inner_pairs = pair.into_inner();
        assert_eq!(inner_pairs.next().unwrap().as_rule(), Rule::identifier);
        assert_eq!(inner_pairs.next().unwrap().as_rule(), Rule::integer);
        assert_eq!(inner_pairs.next().unwrap().as_rule(), Rule::integer);
        assert_eq!(inner_pairs.next().unwrap().as_rule(), Rule::EOI);

        let pair_res = RecurFunctionGrammar::parse(Rule::query, "addition addition");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "100 addition");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "100");
        assert!(pair_res.is_err());

        let pair_res = RecurFunctionGrammar::parse(Rule::projection, "");
        assert!(pair_res.is_err());

        Ok(())
    }
}
