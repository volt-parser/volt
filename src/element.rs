use {
    std::fmt::{self, Display, Formatter},
    crate::rule::RuleId,
};

#[derive(Clone)]
pub enum Element {
    Expression(Expression),
    Choice(Vec<Element>),
    Sequence(Vec<Element>),
    Loop(Box<Element>, LoopRange),
}

impl Element {
    pub fn has_left_recursion(&self, rule_id: &RuleId) -> bool {
        match self {
            Element::Choice(elems) | Element::Sequence(elems) => match elems.get(0) {
                Some(first_elem) => first_elem.has_left_recursion(rule_id),
                None => false,
            },
            Element::Expression(expr) => match expr {
                Expression::Rule(id) => *rule_id == *id,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Element::Expression(expr) => expr.to_string(),
            Element::Choice(elems) => format!("({})", elems.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" / ")),
            Element::Sequence(elems) => format!("({})", elems.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ")),
            Element::Loop(elem, range) => format!("{}{}", elem, range),
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub enum Expression {
    Rule(RuleId),
    String(String),
    Wildcard,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Expression::Rule(id) => id.to_string(),
            Expression::String(value) => format!("\"{}\"", value),
            Expression::Wildcard => "_".to_string(),
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub enum LoopRange {
    ZeroOrMore,
    OneOrMore,
    NToM(usize, usize),
}

impl Display for LoopRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            LoopRange::ZeroOrMore => "*".to_string(),
            LoopRange::OneOrMore => "+".to_string(),
            LoopRange::NToM(n, m) => if *n == 0 && *m == 1 { "?".to_string() } else { format!("{{{},{}}}", n, m).to_string() },
        };

        write!(f, "{}", s)
    }
}

pub fn str(s: &str) -> Element {
    if s.len() == 0 {
        panic!("Empty string is not allowed.");
    }

    Element::Expression(Expression::String(s.to_string()))
}

pub fn wildcard() -> Element {
    Element::Expression(Expression::Wildcard)
}
