use crate::parser::ParserInput;

#[macro_export]
macro_rules! tree {
    ($root:expr) => {
        match $root {
            SyntaxChild::Node(node) => SyntaxTree::new(node),
            _ => panic!("cannot set leaf as syntax tree root"),
        }
    };
}

#[macro_export]
macro_rules! node {
    ($name:expr => $children:expr) => {
        SyntaxChild::node($name.to_string(), $children)
    };
}

#[macro_export]
macro_rules! leaf {
    ($start:expr, $value:expr) => {
        SyntaxChild::leaf($start, $value.to_string())
    };

    ($value:expr) => {
        SyntaxChild::leaf(pos!(usize::MAX, usize::MAX, usize::MAX), $value.to_string())
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr, $children:expr) => {
        SyntaxChild::error($message.to_string(), $children)
    };
}

#[macro_export]
macro_rules! pos {
    ($index:expr, $line:expr, $column:expr) => {
        InputPosition::new($index, $line, $column)
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxTree {
    pub root: SyntaxNode,
}

impl SyntaxTree {
    pub fn new(root: SyntaxNode) -> SyntaxTree {
        SyntaxTree {
            root,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxChild {
    Node(SyntaxNode),
    Leaf(SyntaxLeaf),
    // todo: add position
    Error(SyntaxError),
}

impl SyntaxChild {
    pub fn node(name: String, children: Vec<SyntaxChild>) -> SyntaxChild {
        SyntaxChild::Node(SyntaxNode::new(name, children))
    }

    pub fn leaf(start: InputPosition, value: String) -> SyntaxChild {
        SyntaxChild::Leaf(SyntaxLeaf::new(start, value))
    }

    pub fn error(message: String, children: Vec<SyntaxChild>) -> SyntaxChild {
        SyntaxChild::Error(SyntaxError::new(message, children))
    }

    pub fn join_children(&self) -> String {
        let mut s = String::new();

        match self {
            SyntaxChild::Node(node) => for each_child in &node.children {
                s += &each_child.join_children()
            },
            SyntaxChild::Leaf(leaf) => s += &leaf.value,
            SyntaxChild::Error(_) => (),
        }

        s
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxNode {
    pub name: String,
    pub children: Vec<SyntaxChild>,
}

impl SyntaxNode {
    pub fn new(name: String, children: Vec<SyntaxChild>) -> SyntaxNode {
        SyntaxNode {
            name,
            children,
        }
    }

    // pub fn child_at(&self, i: usize) -> &SyntaxChild {
    //     match self.children.get(i) {
    //         Some(v) => v,
    //         None => panic!("Child index is invalid."),
    //     }
    // }

    // todo: add node_at()
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxLeaf {
    pub start: InputPosition,
    pub value: String,
}

impl SyntaxLeaf {
    pub fn new(start: InputPosition, value: String) -> SyntaxLeaf {
        SyntaxLeaf {
            start,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxError {
    pub message: String,
    pub children: Vec<SyntaxChild>,
}

impl SyntaxError {
    pub fn new(message: String, children: Vec<SyntaxChild>) -> SyntaxError {
        SyntaxError {
            message,
            children,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputPositionCounter {
    pub(crate) lines: Vec<(usize, usize)>,
}

impl From<&str> for InputPositionCounter {
    fn from(input: &str) -> InputPositionCounter {
        let mut lines = Vec::new();
        let mut latest_line_start = 0;

        for (i, ch) in input.chars().enumerate() {
            if ch == '\n' {
                let new_line_start = i + 1;
                lines.push((latest_line_start, new_line_start - latest_line_start));
                latest_line_start = new_line_start;
            }
        }

        lines.push((latest_line_start, input.count() - latest_line_start));

        InputPositionCounter {
            lines,
        }
    }
}

impl InputPositionCounter {
    pub fn get_position(&self, index: usize) -> InputPosition {
        let mut line = 0;
        let mut column = 0;

        for (each_line, (each_line_start, each_line_len)) in self.lines.iter().enumerate() {
            if index < each_line_start + each_line_len {
                line = each_line;
                column = index - each_line_start;
                break;
            }
        }

        InputPosition {
            index,
            line,
            column,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputPosition {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl PartialEq for InputPosition {
    fn eq(&self, other: &Self) -> bool {
        if self.index == usize::MAX || self.line == usize::MAX || self.column == usize::MAX ||
                other.index == usize::MAX || other.line == usize::MAX || other.column == usize::MAX {
            true
        } else {
            self.index == other.index && self.line == other.line && self.column == other.column
        }
    }
}

impl InputPosition {
    pub fn new(index: usize, line: usize, column: usize) -> InputPosition {
        InputPosition {
            index,
            line,
            column,
        }
    }
}

pub trait Expandable {
    fn expand(self, hierarchy: usize, recursive: bool) -> Vec<SyntaxChild>;
}

impl Expandable for Vec<SyntaxChild> {
    fn expand(self, hierarchy: usize, recursive: bool) -> Vec<SyntaxChild> {
        let mut children: Vec<SyntaxChild> = Vec::new();

        for each_child in self {
            match each_child {
                SyntaxChild::Node(node) if hierarchy == 0 || recursive => children.append(&mut node.children.expand(hierarchy + 1, recursive)),
                _ => children.push(each_child),
            }
        }

        children
    }
}
