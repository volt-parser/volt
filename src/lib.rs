pub mod element;
pub mod parser;
pub mod rule;
pub mod tree;
#[cfg(test)]
mod tests;

use {
    std::collections::HashMap,
    element::*,
    parser::*,
    rule::*,
};

#[macro_export]
macro_rules! add_rules {
    ($($rule_name:ident $([$separator:expr])? := $rule_elem:expr;)*) => {
        {
            Self {
                $(
                    $rule_name: $rule_elem $(.separate($separator, true))?,
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! choice {
    ($($element:expr),+ $(,)?) => {
        Element::Choice(vec![$($element,)+])
    };
}

pub struct Volt {
    // todo: Optimize process speed of HashMap.
    rule_map: HashMap<RuleId, Element>,
    max_recursion: usize,
}

impl Volt {
    pub fn new(rule_map: HashMap<RuleId, Element>, max_recursion: usize) -> Volt {
        Volt {
            rule_map,
            max_recursion,
        }
    }

    pub fn add_module<T: ModuleAssist>(&mut self, module: T) {
        let rules: Vec<Rule> = module.into_rule_vec().into();

        for each_rule in rules {
            let id = each_rule.id.clone();

            if self.rule_map.contains_key(&id) {
                panic!("Rule ID `{}` is already declared.", id);
            }

            self.rule_map.insert(id, each_rule.element);
        }
    }

    pub fn parse(&self, input: &str, entry_rule_id: &RuleId) -> ParserResult {
        Parser::parse(&self, input, entry_rule_id)
    }
}

pub trait Module: ModuleAssist {
    fn new() -> Self;
}

pub trait ModuleAssist {
    fn into_rule_vec(self) -> RuleVec;
}
