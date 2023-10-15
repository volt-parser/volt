use {
    std::fmt::{self, Display, Formatter},
    regex::Regex,
    crate::*,
    crate::rule::RuleId,
    crate::tree::SyntaxChild,
};

#[derive(Clone)]
pub enum Element {
    Expression(Expression),
    Choice(Vec<Element>),
    Sequence(Vec<Element>),
    Loop(Box<Element>, LoopRange),
    PositiveLookahead(Box<Element>),
    NegativeLookahead(Box<Element>),
    Error(Box<Element>, String),
    Catch(Box<Element>, String),
    CatchSkip(Box<Element>, String, Box<Element>),
    TreeReduction(Box<Element>, fn(Vec<SyntaxChild>) -> Vec<SyntaxChild>),
    Group(Box<Element>, String),
    Expansion(Box<Element>),
    ExpansionOnce(Box<Element>),
    Join(Box<Element>),
    Hidden(Box<Element>),
}

impl Element {
    fn range(self, range: LoopRange) -> Element {
        Element::Loop(Box::new(self), range)
    }

    pub fn times(self, n: usize) -> Element {
        self.min_max(n, n)
    }

    pub fn min_max(self, min: usize, max: usize) -> Element {
        self.range(LoopRange::min_max(min, max))
    }

    pub fn min(self, min: usize) -> Element {
        self.range(LoopRange::min(min))
    }

    pub fn max(self, max: usize) -> Element {
        self.range(LoopRange::max(max))
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

    pub fn err(self, message: &str) -> Element {
        Element::Error(Box::new(self), message.to_string())
    }

    pub fn catch(self, message: &str) -> Element {
        Element::Catch(Box::new(self), message.to_string())
    }

    pub fn catch_to(self, message: &str, to: Element) -> Element {
        Element::CatchSkip(Box::new(self), message.to_string(), Box::new(to))
    }

    pub fn reduce(self, reducer: fn(Vec<SyntaxChild>) -> Vec<SyntaxChild>) -> Element {
        Element::TreeReduction(Box::new(self), reducer)
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

    pub fn join(self) -> Element {
        Element::Join(Box::new(self))
    }

    pub fn hide(self) -> Element {
        Element::Hidden(Box::new(self))
    }

    pub fn around(self, enclosure: Element) -> Element {
        seq![enclosure.clone(), self, enclosure]
    }

    pub fn separate(self, separator: Element) -> Element {
        seq![self.clone(), seq![separator.clone(), self].min(0), separator.optional()]
    }

    pub fn separate_times(self, separator: Element, loop_range: LoopRange) -> Element {
        seq![self.clone(), seq![separator.clone(), self].range(loop_range), separator.optional()]
    }

    pub fn separate_around(self, separator: Element) -> Element {
        seq![separator.clone().optional(), self.clone(), seq![separator.clone(), self].min(0), separator.optional()]
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
            Element::Error(elem, message) => format!("{}.err({})", elem, message),
            Element::Catch(elem, message) => format!("{}.catch({})", elem, message),
            Element::CatchSkip(elem, message, to) => format!("{}.catch_to({}, {})", elem, to, message),
            Element::TreeReduction(elem, _) => format!("{}.reduce", elem),
            Element::Group(elem, name) => format!("{}#{}", elem, name),
            Element::Expansion(elem) | Element::ExpansionOnce(elem) => format!("{}###", elem),
            Element::Join(elem) => format!("{}.join", elem),
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Maxable {
    Max(usize),
    NoLimit,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn times(n: usize) -> LoopRange {
        LoopRange::min_max(n, n)
    }

    pub fn min_max(min: usize, max: usize) -> LoopRange {
        LoopRange {
            min,
            max: Maxable::Max(max),
        }
    }

    pub fn min(min: usize) -> LoopRange {
        LoopRange {
            min,
            max: Maxable::NoLimit,
        }
    }

    pub fn max(max: usize) -> LoopRange {
        LoopRange {
            min: 0,
            max: Maxable::Max(max),
        }
    }

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
