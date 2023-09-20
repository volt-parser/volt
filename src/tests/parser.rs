use {
    crate::*,
    crate::parser::*,
    crate::tree::*,
    volt_derive::RuleContainer,
    speculate::speculate,
};

speculate!{
    before {
        let volt = &mut Volt::new();
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

    /* Choice Element */

    it "choice consumes characters as much its children 1" {
        expect_failure("", "TestModule::choice", ParserError::NoMatchedRule);
    }

    it "choice consumes characters as much its children 2" {
        expect_failure("ab", "TestModule::choice", ParserError::NoMatchedRule);
    }

    it "choices first choice when match" {
        expect_success("a", "TestModule::choice", tree!{
            node!{
                "TestModule::choice" => vec![
                    leaf!("a"),
                ]
            }
        });
    }

    it "choices the next choice when first choice doesn't match" {
        expect_success("b", "TestModule::choice", tree!{
            node!{
                "TestModule::choice" => vec![
                    leaf!("b"),
                ]
            }
        });
    }

    it "choice doesn't match element not exist in children" {
        expect_failure("c", "TestModule::choice", ParserError::NoMatchedRule);
    }

    /* Sequence Element */

    it "sequence consumes characters as much its children 1" {
        expect_failure("a", "TestModule::sequence", ParserError::NoMatchedRule);
    }

    it "sequence consumes characters as much its children 2" {
        expect_failure("abc", "TestModule::sequence", ParserError::NoMatchedRule);
    }

    it "sequence matches completely same input 1" {
        expect_success("ab", "TestModule::sequence", tree!{
            node!{
                "TestModule::sequence" => vec![
                    leaf!("a"),
                    leaf!("b"),
                ]
            }
        });
    }

    it "sequence matches completely same input 2" {
        expect_failure("ac", "TestModule::sequence", ParserError::NoMatchedRule);
    }

    /* String Expression */

    it "string consumes characters as much its length 1" {
        expect_failure("a", "TestModule::string", ParserError::NoMatchedRule);
    }

    it "string consumes characters as much its length 2" {
        expect_failure("abc", "TestModule::string", ParserError::NoMatchedRule);
    }

    it "string generates single leaf" {
        expect_success("ab", "TestModule::string", tree!{
            node!{
                "TestModule::string" => vec![
                    leaf!("ab"),
                ]
            }
        });
    }

    it "string supports multibyte characters" {
        expect_success("あい", "TestModule::multibyte_string", tree!{
            node!{
                "TestModule::multibyte_string" => vec![
                    leaf!("あい"),
                ]
            }
        });
    }

    /* Wildcard Expression */

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
    choice: Element,
    sequence: Element,
    string: Element,
    multibyte_string: Element,
    wildcard: Element,
}

impl Module for TestModule {
    fn new() -> TestModule {
        add_rules!{
            // left_recursion := TestModule::left_recursion();
            choice := choice![str("a"), str("b")];
            sequence := seq![str("a"), str("b")];
            string := str("ab");
            multibyte_string := str("あい");
            wildcard := wildcard();
        }
    }
}
