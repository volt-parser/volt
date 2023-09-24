use {
    std::fmt::{self, Display, Formatter},
    regex::Regex,
    crate::*,
    crate::rule::RuleId,
};

#[derive(Clone)]
pub enum Element {
    Expression(Expression),
    Choice(Vec<Element>),
    Sequence(Vec<Element>),
    Loop(Box<Element>, LoopRange),
    PositiveLookahead(Box<Element>),
    NegativeLookahead(Box<Element>),
    Group(Box<Element>, String),
    Expansion(Box<Element>),
    ExpansionOnce(Box<Element>),
    Hidden(Box<Element>),
}

impl Element {
    pub fn times(self, n: usize) -> Element {
        self.min_max(n, n)
    }

    pub fn min_max(self, min: usize, max: usize) -> Element {
        let range = LoopRange {
            min,
            max: Maxable::Max(max),
        };

        Element::Loop(Box::new(self), range)
    }

    pub fn min(self, min: usize) -> Element {
        let range = LoopRange {
            min,
            max: Maxable::NoLimit,
        };

        Element::Loop(Box::new(self), range)
    }

    pub fn max(self, max: usize) -> Element {
        let range = LoopRange {
            min: 0,
            max: Maxable::Max(max),
        };

        Element::Loop(Box::new(self), range)
    }

    pub fn optional(self) -> Element {
        self.min_max(0, 1)
    }

    pub fn poslook(self) -> Element {
        Element::PositiveLookahead(Box::new(self))
    }

    pub fn neglook(self) -> Element {
        Element::NegativeLookahead(Box::new(self))
    }

    pub fn group(self, name: &str) -> Element {
        Element::Group(Box::new(self), name.to_string())
    }

    pub fn expand(self) -> Element {
        Element::Expansion(Box::new(self))
    }

    pub fn expand_once(self) -> Element {
        Element::ExpansionOnce(Box::new(self))
    }

    pub fn hide(self) -> Element {
        Element::Hidden(Box::new(self))
    }

    pub fn separate(self, separator: Element) -> Element {
        seq![self.clone(), seq![separator.clone(), self].min(0), separator.optional()]
    }

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
            Element::PositiveLookahead(elem) => format!("&{}", elem),
            Element::NegativeLookahead(elem) => format!("!{}", elem),
            Element::Group(elem, name) => format!("{}#{}", elem, name),
            Element::Expansion(elem) | Element::ExpansionOnce(elem) => format!("{}###", elem),
            Element::Hidden(elem) => format!("{}##", elem),
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub enum Expression {
    Rule(RuleId),
    String(String),
    CharacterClass(Regex),
    Wildcard,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Expression::Rule(id) => id.to_string(),
            Expression::String(v) => format!("\"{}\"", v),
            Expression::CharacterClass(v) => format!("{}", v),
            Expression::Wildcard => "_".to_string(),
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Maxable {
    Max(usize),
    NoLimit,
}

#[derive(Clone)]
pub struct LoopRange {
    pub min: usize,
    pub max: Maxable,
}

impl Display for LoopRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.min {
            0 => match &self.max {
                Maxable::Max(v) if *v == 1 => return write!(f, "_"),
                Maxable::NoLimit => return write!(f, "*"),
                _ => (),
            },
            1 => match &self.max {
                Maxable::NoLimit => return write!(f, "*"),
                _ => (),
            },
            _ => (),
        };

        let max = match &self.max {
            Maxable::Max(v) => v.to_string(),
            Maxable::NoLimit => String::new(),
        };

        write!(f, "{{{},{}}}", self.min, max)
    }
}

impl LoopRange {
    pub fn is_single_times(&self) -> bool {
        self.min == 1 && self.max == Maxable::Max(1)
    }
}

pub fn str(s: &str) -> Element {
    if s.len() == 0 {
        panic!("Empty string is not allowed.");
    }

    Element::Expression(Expression::String(s.to_string()))
}

pub fn chars(s: &str) -> Element {
    // todo: check pattern
    let patt = format!("[{}]", s.replace("[", "\\[").replace("]", "\\]"));

    let regex = if let Ok(v) = Regex::new(&patt) {
        v
    } else {
        panic!("invalid regex pattern");
    };

    Element::Expression(Expression::CharacterClass(regex))
}

pub fn wildcard() -> Element {
    Element::Expression(Expression::Wildcard)
}
