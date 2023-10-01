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
    ($name:expr => [$($child:expr),* $(,)?]) => {
        SyntaxChild::node($name.to_string(), vec![$($child),*])
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
    ($message:expr, [$($child:expr),* $(,)?]) => {
        SyntaxChild::error($message.to_string(), vec![$($child),*])
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

pub trait SyntaxChildVec {
    fn expand(self, hierarchy: usize, recursive: bool) -> Vec<SyntaxChild>;

    fn get_start_position(&self) -> Option<InputPosition>;

    fn eject_errors(self) -> Vec<SyntaxChild>;

    fn join_into_string(&self) -> String;

    fn get_child(&self, index: usize) -> &SyntaxChild;

    // add: search_node()
    fn get_node(&self, index: usize) -> &SyntaxNode;

    fn get_leaf(&self, index: usize) -> &SyntaxLeaf;

    fn get_error(&self, index: usize) -> &SyntaxError;
}

impl SyntaxChildVec for Vec<SyntaxChild> {
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

    fn get_start_position(&self) -> Option<InputPosition> {
        for each_child in self {
            match each_child {
                SyntaxChild::Node(node) => if let Some(v) = node.children.get_start_position() {
                    return Some(v);
                },
                SyntaxChild::Leaf(leaf) => return Some(leaf.start.clone()),
                SyntaxChild::Error(err) => if let Some(v) = err.children.get_start_position() {
                    return Some(v);
                },
            }
        }

        None
    }

    fn eject_errors(self) -> Vec<SyntaxChild> {
        let mut errors = Vec::new();

        for each_child in self {
            match each_child {
                SyntaxChild::Node(node) => errors.append(&mut node.children.eject_errors()),
                SyntaxChild::Error(err) => errors.push(SyntaxChild::Error(err)),
                _ => (),
            }
        }

        errors
    }

    fn join_into_string(&self) -> String {
        let mut value = String::new();

        for each_child in self {
            match each_child {
                SyntaxChild::Node(node) => value += &node.children.join_into_string(),
                SyntaxChild::Leaf(leaf) => value += &leaf.value,
                SyntaxChild::Error(_) => (),
            }
        }

        value
    }

    fn get_child(&self, index: usize) -> &SyntaxChild {
        if let Some(v) = self.get(index) {
            v
        } else {
            panic!("syntax child index is out of range");
        }
    }

    // add: search_node()
    fn get_node(&self, index: usize) -> &SyntaxNode {
        if let SyntaxChild::Node(node) = self.get_child(index) {
            node
        } else {
            panic!("expected syntax node");
        }
    }

    fn get_leaf(&self, index: usize) -> &SyntaxLeaf {
        if let SyntaxChild::Leaf(leaf) = self.get_child(index) {
            leaf
        } else {
            panic!("expected syntax leaf");
        }
    }

    // add: search_error()
    fn get_error(&self, index: usize) -> &SyntaxError {
        if let SyntaxChild::Error(error) = self.get_child(index) {
            error
        } else {
            panic!("expected syntax error");
        }
    }
}
