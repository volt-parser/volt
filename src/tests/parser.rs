use {
    crate::*,
    crate::parser::*,
    crate::tree::*,
    volt_derive::RuleContainer,
    speculate::speculate,
};

speculate!{
    before {
        let volt = &mut Volt::new(HashMap::new(), 1024);
        volt.add_module(TestModule::new());

        let assert_ast = |input: &str, rule_id: &str, expected: ParserResult|
            assert_eq!(Parser::parse(volt, input, &RuleId(rule_id.to_string())), expected);

        #[allow(unused)]
        let expect_success = |input: &str, rule_id: &str, expected: SyntaxTree|
            assert_ast(input, rule_id, Ok(expected));

        #[allow(unused)]
        let expect_failure = |input: &str, rule_id: &str, expected: ParserError|
            assert_ast(input, rule_id, Err(expected));
    }

    // it "detect max recursion excess" {
    //     expect_failure("", "TestModule::left_recursion", ParserError::ExceededMaxRecursion);
    // }

    it "wildcard consumes single character 1" {
        expect_failure("", "TestModule::wildcard", ParserError::NoMatchedRule);
    }

    it "wildcard consumes single character 2" {
        expect_failure("aa", "TestModule::wildcard", ParserError::NoMatchedRule);
    }

    it "wildcard generates single leaf" {
        expect_success("a", "TestModule::wildcard", tree!{
            node!{
                "TestModule::wildcard" => vec![
                    leaf!("a"),
                ]
            }
        });
    }

    it "wildcard treats single multibyte character as a character" {
        expect_success("あ", "TestModule::wildcard", tree!{
            node!{
                "TestModule::wildcard" => vec![
                    leaf!("あ"),
                ]
            }
        });
    }
}

#[derive(RuleContainer)]
struct TestModule {
    // left_recursion: Element,
    wildcard: Element,
}

impl Module for TestModule {
    fn new() -> TestModule {
        add_rules!{
            // left_recursion := TestModule::left_recursion();
            wildcard := wildcard();
        }
    }
}
