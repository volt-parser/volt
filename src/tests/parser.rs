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

    describe "choice element" {
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
    }

    describe "sequence element" {
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
    }

    describe "loop element" {
        describe "n times" {
            it "repeats for the number of times in the specified range 1-1" {
                expect_failure("a", "TestModule::loop_range1", ParserError::NoMatchedRule);
            }

            it "repeats for the number of times in the specified range 1-2" {
                expect_success("aa", "TestModule::loop_range1", tree!{
                    node!{
                        "TestModule::loop_range1" => vec![
                            leaf!("a"),
                            leaf!("a"),
                        ]
                    }
                });
            }

            it "repeats for the number of times in the specified range 1-3" {
                expect_failure("aaa", "TestModule::loop_range1", ParserError::NoMatchedRule);
            }
        }

        describe "min" {
            it "repeats for the number of times in the specified range 2-1" {
                expect_failure("", "TestModule::loop_range2", ParserError::NoMatchedRule);
            }

            it "repeats for the number of times in the specified range 2-2" {
                expect_success("a", "TestModule::loop_range2", tree!{
                    node!{
                        "TestModule::loop_range2" => vec![
                            leaf!("a"),
                        ]
                    }
                });
            }

            it "repeats for the number of times in the specified range 2-3" {
                expect_success("aa", "TestModule::loop_range2", tree!{
                    node!{
                        "TestModule::loop_range2" => vec![
                            leaf!("a"),
                            leaf!("a"),
                        ]
                    }
                });
            }
        }

        describe "max" {
            it "repeats for the number of times in the specified range 3-1" {
                expect_success("", "TestModule::loop_range3", tree!{
                    node!{
                        "TestModule::loop_range3" => vec![]
                    }
                });
            }

            it "repeats for the number of times in the specified range 3-2" {
                expect_success("a", "TestModule::loop_range3", tree!{
                    node!{
                        "TestModule::loop_range3" => vec![
                            leaf!("a"),
                        ]
                    }
                });
            }

            it "repeats for the number of times in the specified range 3-3" {
                expect_failure("aa", "TestModule::loop_range3", ParserError::NoMatchedRule);
            }
        }
    }

    describe "positive lookahead element" {
        it "doesn't change input index 1" {
            expect_success("a", "TestModule::poslook", tree!{
                node!{
                    "TestModule::poslook" => vec![
                        leaf!("a"),
                    ]
                }
            });
        }

        it "doesn't change input index 2" {
            expect_failure("b", "TestModule::poslook", ParserError::NoMatchedRule);
        }
    }

    describe "negative lookahead element" {
        it "doesn't change input index 1" {
            expect_failure("a", "TestModule::neglook", ParserError::NoMatchedRule);
        }

        it "doesn't change input index 2" {
            expect_success("b", "TestModule::neglook", tree!{
                node!{
                    "TestModule::neglook" => vec![
                        leaf!("b"),
                    ]
                }
            });
        }
    }

    describe "error element" {
        it "generates error when matched" {
            expect_success("a", "TestModule::error", tree!{
                node!{
                    "TestModule::error" => vec![
                        error!("msg", vec![
                            leaf!("a"),
                        ]),
                    ]
                }
            });
        }

        it "doesn't add input index when not matched" {
            expect_success("", "TestModule::error", tree!{
                node!{
                    "TestModule::error" => vec![]
                }
            });
        }
    }

    describe "error skip element" {
        it "successes parsing normally" {
            expect_success("a;", "TestModule::error_to", tree!{
                node!{
                    "TestModule::error_to" => vec![
                        leaf!("a"),
                        leaf!(";"),
                    ]
                }
            });
        }

        it "adds input index until end string on failure" {
            expect_success("b;", "TestModule::error_to", tree!{
                node!{
                    "TestModule::error_to" => vec![
                        error!("msg", vec![
                            leaf!(";"),
                        ]),
                    ]
                }
            });
        }

        it "try parsing until end of input" {
            expect_failure("b", "TestModule::error_to", ParserError::NoMatchedRule);
        }
    }

    describe "group element" {
        it "group a sequence" {
            expect_success("aa", "TestModule::sequence_group", tree!{
                node!{
                    "TestModule::sequence_group" => vec![
                        node!{
                            "group" => vec![
                                leaf!("a"),
                                leaf!("a"),
                            ]
                        }
                    ]
                }
            });
        }

        it "group an expression" {
            expect_success("a", "TestModule::expression_group", tree!{
                node!{
                    "TestModule::expression_group" => vec![
                        node!{
                            "group" => vec![
                                leaf!("a"),
                            ]
                        }
                    ]
                }
            });
        }
    }

    describe "expansion element" {
        it "expands children at all levels of hierarchy" {
            expect_success("abc", "TestModule::expansion", tree!{
                node!{
                    "TestModule::expansion" => vec![
                        leaf!("a"),
                        leaf!("b"),
                        leaf!("c"),
                    ]
                }
            });
        }

        it "expands children at the first level of hierarchy" {
            expect_success("abc", "TestModule::expansion_once", tree!{
                node!{
                    "TestModule::expansion_once" => vec![
                        leaf!("a"),
                        leaf!("b"),
                        node!{
                            "group_b" => vec![
                                leaf!("c"),
                            ]
                        },
                    ]
                }
            });
        }
    }

    describe "hidden element" {
        it "element shouldn't reflected in AST" {
            expect_success("a", "TestModule::hidden", tree!{
                node!{
                    "TestModule::hidden" => vec![]
                }
            });
        }
    }

    describe "separated element" {
        it "should contain at least one item" {
            expect_failure("", "TestModule::separated", ParserError::NoMatchedRule);
        }

        it "can put single item" {
            expect_success("a", "TestModule::separated", tree!{
                node!{
                    "TestModule::separated" => vec![
                        leaf!("a"),
                    ]
                }
            });
        }

        it "can put a separator at the last of single item" {
            expect_success("a,", "TestModule::separated", tree!{
                node!{
                    "TestModule::separated" => vec![
                        leaf!("a"),
                        leaf!(","),
                    ]
                }
            });
        }

        it "can put multiple items" {
            expect_success("a,a", "TestModule::separated", tree!{
                node!{
                    "TestModule::separated" => vec![
                        leaf!("a"),
                        leaf!(","),
                        leaf!("a"),
                    ]
                }
            });
        }

        it "can put a separator at the last of multiple items" {
            expect_success("a,a,", "TestModule::separated", tree!{
                node!{
                    "TestModule::separated" => vec![
                        leaf!("a"),
                        leaf!(","),
                        leaf!("a"),
                        leaf!(","),
                    ]
                }
            });
        }

        it "separators can be hidden" {
            expect_success("a,a", "TestModule::separated_with_hidden_separator", tree!{
                node!{
                    "TestModule::separated_with_hidden_separator" => vec![
                        leaf!("a"),
                        leaf!("a"),
                    ]
                }
            });
        }
    }

    describe "string expression" {
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
    }

    describe "character class expression" {
        it "matches a specified character 1" {
            expect_success("a", "TestModule::character_class1", tree!{
                node!{
                    "TestModule::character_class1" => vec![
                        leaf!("a"),
                    ]
                }
            });
        }

        it "matches a specified character 2" {
            expect_success("b", "TestModule::character_class1", tree!{
                node!{
                    "TestModule::character_class1" => vec![
                        leaf!("b"),
                    ]
                }
            });
        }

        it "matches a specified character 3" {
            expect_failure("c", "TestModule::character_class1", ParserError::NoMatchedRule);
        }

        it "consumes only one character" {
            expect_failure("aa", "TestModule::character_class1", ParserError::NoMatchedRule);
        }

        it "supports number specification" {
            expect_success("0", "TestModule::character_class2", tree!{
                node!{
                    "TestModule::character_class2" => vec![
                        leaf!("0"),
                    ]
                }
            });
        }

        it "supports regex pattern enclosure" {
            expect_success("[", "TestModule::character_class3", tree!{
                node!{
                    "TestModule::character_class3" => vec![
                        leaf!("["),
                    ]
                }
            });
        }
    }

    describe "wildcard expression" {
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
}

#[derive(RuleContainer)]
struct TestModule {
    // left_recursion: Element,
    choice: Element,
    sequence: Element,
    loop_range1: Element,
    loop_range2: Element,
    loop_range3: Element,
    poslook: Element,
    neglook: Element,
    error: Element,
    error_to: Element,
    sequence_group: Element,
    expression_group: Element,
    expansion: Element,
    expansion_once: Element,
    hidden: Element,
    separated: Element,
    separated_with_hidden_separator: Element,
    string: Element,
    multibyte_string: Element,
    character_class1: Element,
    character_class2: Element,
    character_class3: Element,
    wildcard: Element,
}

impl Module for TestModule {
    fn new() -> TestModule {
        add_rules!{
            // left_recursion := TestModule::left_recursion();
            choice := choice![str("a"), str("b")];
            sequence := seq![str("a"), str("b")];
            loop_range1 := seq![wildcard().times(2)];
            loop_range2 := seq![wildcard().min(1)];
            loop_range3 := seq![wildcard().max(1)];
            poslook := seq![str("a").poslook(), wildcard()];
            neglook := seq![str("a").neglook(), wildcard()];
            error := str("a").err("msg");
            error_to := seq![str("a"), str(";")].err_to("msg", str(";"));
            sequence_group := seq![wildcard(), wildcard()].group("group");
            expression_group := wildcard().group("group");
            expansion := seq![wildcard(), seq![wildcard(), seq![wildcard()].group("group_b")].group("group_a").expand()];
            expansion_once := seq![wildcard(), seq![wildcard(), seq![wildcard()].group("group_b")].group("group_a").expand_once()];
            hidden := wildcard().hide();
            separated := wildcard().separate(str(","));
            separated_with_hidden_separator := wildcard().separate(str(",").hide());
            string := str("ab");
            multibyte_string := str("あい");
            character_class1 := chars("ab");
            character_class2 := chars(r"\d");
            character_class3 := chars("[");
            wildcard := wildcard();
        }
    }
}
