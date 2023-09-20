use crate::{
    *,
    tree::*,
};

trait ParserInput {
    fn count(&self) -> usize;

    fn slice(&self, skip: usize, take: usize) -> String;
}

impl ParserInput for str {
    fn count(&self) -> usize {
        self.chars().count()
    }

    fn slice(&self, skip: usize, take: usize) -> String {
        self.chars().skip(skip).take(take).collect::<String>()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserError {
    NoMatchedRule,
    RuleNotExists { id: RuleId },
    ExceededMaxRecursion,
}

pub type ParserResult = Result<SyntaxTree, ParserError>;
pub type OptionalParserResult<'a, T> = Result<Option<T>, ParserError>;

pub struct Parser<'a> {
    volt: &'a Volt,
    input: &'a str,
    index: usize,
    pub(crate) recursion: usize,
}

impl<'a> Parser<'a> {
    pub fn parse(volt: &'a Volt, input: &str, entry_rule_id: &RuleId) -> ParserResult {
        let mut parser = Parser {
            volt,
            input,
            index: 0,
            recursion: 0,
        };

        match parser.rule(entry_rule_id)? {
            Some(root) if parser.index == parser.input.count() => Ok(SyntaxTree::new(root)),
            _ => Err(ParserError::NoMatchedRule),
        }
    }

    pub(crate) fn rule(&mut self, rule_id: &RuleId) -> OptionalParserResult<SyntaxNode> {
        if self.recursion >= self.volt.max_recursion {
            return Err(ParserError::ExceededMaxRecursion);
        }

        self.recursion += 1;

        let result = match self.volt.rule_map.get(rule_id) {
            Some(elem) => if let Some(children) = self.element(elem)? { Ok(Some(SyntaxNode::new(rule_id.to_string(), children))) } else { Ok(None) },
            None => Err(ParserError::RuleNotExists { id: rule_id.clone() }),
        };

        self.recursion -= 1;
        result
    }

    pub(crate) fn element(&mut self, elem: &Element) -> OptionalParserResult<Vec<SyntaxChild>> {
        let children = match elem {
            Element::Choice(elems) => self.choice(elems)?,
            Element::Expression(expr) => match expr {
                Expression::Rule(id) => if let Some(child_node) = self.rule(id)? { Some(vec![SyntaxChild::Node(child_node)]) } else { None },
                Expression::String(s) => self.string(s)?,
                Expression::Wildcard => self.wildcard()?,
            },
            _ => unimplemented!(),
        };

        Ok(children)
    }

    pub (crate) fn choice(&mut self, elems: &Vec<Element>) -> OptionalParserResult<Vec<SyntaxChild>> {
        let tmp_index = self.index;

        for each_elem in elems {
            if let Some(children) = self.element(each_elem)? {
                return Ok(Some(children));
            } else {
                self.index = tmp_index;
            }
        }

        Ok(None)
    }

    pub(crate) fn string(&mut self, s: &str) -> OptionalParserResult<Vec<SyntaxChild>> {
        if self.input.count() >= self.index + s.count() && self.input.slice(self.index, s.count()) == *s {
            self.index += s.count();
            Ok(Some(vec![SyntaxChild::leaf(s.to_string())]))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn wildcard(&mut self) -> OptionalParserResult<Vec<SyntaxChild>> {
        if self.input.count() >= self.index + 1 {
            let s = self.input.slice(self.index, 1);
            self.index += 1;
            Ok(Some(vec![SyntaxChild::leaf(s)]))
        } else {
            Ok(None)
        }
    }
}
