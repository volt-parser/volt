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

// todo: rename add_rules
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

#[macro_export]
macro_rules! seq {
    ($($element:expr),+ $(,)?) => {
        Element::Sequence(vec![$($element,)+])
    };
}

pub struct Volt {
    // todo: Optimize process speed of HashMap.
    rule_map: HashMap<RuleId, Element>,
    max_recursion: usize,
}

impl Volt {
    pub fn new() -> Volt {
        Volt {
            rule_map: HashMap::new(),
            max_recursion: 1024,
        }
    }

    pub fn add_module<T: VoltModuleAssist>(&mut self, module: T) {
        let rules: Vec<Rule> = module.into_rule_vec().into();

        for each_rule in rules {
            let id = each_rule.id.clone();

            if self.rule_map.contains_key(&id) {
                panic!("Rule ID `{}` is already declared.", id);
            }

            self.rule_map.insert(id, each_rule.element);
        }
    }

    pub fn set_max_recursion(&mut self, max_recursion: usize) {
        self.max_recursion = max_recursion;
    }

    pub fn parse(&self, input: &str, entry_rule_id: &RuleId) -> ParserResult {
        Parser::parse(&self, input, entry_rule_id)
    }
}

pub trait VoltModule: VoltModuleAssist {
    fn new() -> Self;
}

pub trait VoltModuleAssist {
    fn into_rule_vec(self) -> RuleVec;
}
