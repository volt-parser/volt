use {
    std::fmt::{self, Debug, Display, Formatter},
    crate::element::*,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RuleId(pub String);

impl Display for RuleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Rule {
    pub id: RuleId,
    pub element: Element,
}

impl Rule {
    pub fn new(id: RuleId, element: Element) -> Rule {
        Rule {
            id,
            element,
        }
    }

    pub fn detect_left_recursion(self) -> Rule {
        if self.element.has_left_recursion(&self.id) {
            panic!("Left recursion detected at rule definition of `{}`.", self.id);
        }

        self
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} := {};", self.id, self.element)
    }
}

#[derive(Clone)]
pub struct RuleVec(pub Vec<Rule>);

impl Debug for RuleVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for RuleVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|r| r.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

impl Into<Vec<Rule>> for RuleVec {
    fn into(self) -> Vec<Rule> {
        self.0
    }
}
