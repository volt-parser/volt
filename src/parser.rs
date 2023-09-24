use {
    regex::Regex,
    crate::{
        *,
        tree::*,
    }
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

    fn rule(&mut self, rule_id: &RuleId) -> OptionalParserResult<SyntaxNode> {
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

    fn element(&mut self, elem: &Element) -> OptionalParserResult<Vec<SyntaxChild>> {
        let children = match elem {
            Element::Choice(elems) => self.choice(elems)?,
            Element::Sequence(elems) => self.sequence(elems)?,
            Element::Expression(expr) => match expr {
                Expression::Rule(id) => if let Some(child_node) = self.rule(id)? { Some(vec![SyntaxChild::Node(child_node)]) } else { None },
                Expression::String(s) => self.string(s)?,
                Expression::CharacterClass(v) => self.character_class(v)?,
                Expression::Wildcard => self.wildcard()?,
            },
            Element::Loop(elem, range) => self.times(elem, range)?,
            Element::PositiveLookahead(elem) => self.lookahead(elem, true)?,
            Element::NegativeLookahead(elem) => self.lookahead(elem, false)?,
            Element::Group(elem, name) => self.element(elem)?.map(|children| vec![SyntaxChild::Node(SyntaxNode::new(name.to_string(), children))]),
            Element::Hidden(elem) => self.element(elem)?.map(|_| vec![]),
        };

        Ok(children)
    }

    fn choice(&mut self, elems: &Vec<Element>) -> OptionalParserResult<Vec<SyntaxChild>> {
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

    fn sequence(&mut self, elems: &Vec<Element>) -> OptionalParserResult<Vec<SyntaxChild>> {
        let tmp_index = self.index;
        let mut children = Vec::new();

        for each_elem in elems {
            match self.element(each_elem)? {
                Some(mut new_children) => children.append(&mut new_children),
                None => {
                    self.index = tmp_index;
                    return Ok(None);
                },
            }
        }

        Ok(Some(children))
    }

    fn times(&mut self, elem: &Element, range: &LoopRange) -> OptionalParserResult<Vec<SyntaxChild>> {
        if range.is_single_times() {
            self.element(elem)
        } else {
            let tmp_index = self.index;
            let mut children = Vec::new();
            let mut count = 0;

            loop {
                let mut new_children = match self.element(elem) {
                    Ok(option) => match option {
                        Some(new_children) => new_children,
                        _ => break,
                    },
                    Err(e) => return Err(e),
                };

                match &range.max {
                    Maxable::Max(max) => {
                        if count <= *max {
                            children.append(&mut new_children);
                            count += 1;

                            if count == *max {
                                break;
                            }
                        } else {
                            break;
                        }
                    },
                    Maxable::NoLimit => {
                        children.append(&mut new_children);
                        count += 1;
                    },
                }
            }

            if count >= range.min {
                Ok(Some(children))
            } else {
                self.index = tmp_index;
                Ok(None)
            }
        }
    }

    fn lookahead(&mut self, elem: &Element, is_positive: bool) -> OptionalParserResult<Vec<SyntaxChild>> {
        let tmp_index = self.index;
        let result = self.element(elem);

        match result {
            Ok(option) => {
                self.index = tmp_index;

                let has_succeeded = if is_positive {
                    option.is_some()
                } else {
                    option.is_none()
                };

                if has_succeeded {
                    Ok(Some(Vec::new()))
                } else {
                    Ok(None)
                }
            },
            Err(e) => Err(e),
        }
    }

    fn string(&mut self, s: &str) -> OptionalParserResult<Vec<SyntaxChild>> {
        if self.input.count() >= self.index + s.count() && self.input.slice(self.index, s.count()) == *s {
            self.index += s.count();
            Ok(Some(vec![SyntaxChild::leaf(s.to_string())]))
        } else {
            Ok(None)
        }
    }

    fn character_class(&mut self, regex: &Regex) -> OptionalParserResult<Vec<SyntaxChild>> {
        if self.input.count() >= self.index + 1 {
            // todo: optimization
            let target = self.input.slice(self.index, 1);

            match regex.find(&target) {
                Some(regex_match) if regex_match.start() == 0 => {
                    self.index += 1;
                    Ok(Some(vec![SyntaxChild::leaf(target)]))
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn wildcard(&mut self) -> OptionalParserResult<Vec<SyntaxChild>> {
        if self.input.count() >= self.index + 1 {
            let s = self.input.slice(self.index, 1);
            self.index += 1;
            Ok(Some(vec![SyntaxChild::leaf(s)]))
        } else {
            Ok(None)
        }
    }
}
