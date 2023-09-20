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
    ($value:expr) => {
        SyntaxChild::leaf($value.to_string())
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
    Continuation,
}

impl SyntaxChild {
    pub fn node(name: String, children: Vec<SyntaxChild>) -> SyntaxChild {
        SyntaxChild::Node(SyntaxNode::new(name, children))
    }

    pub fn leaf(value: String) -> SyntaxChild {
        SyntaxChild::Leaf(SyntaxLeaf::new(value))
    }

    pub fn join_children(&self) -> String {
        let mut s = String::new();

        match self {
            SyntaxChild::Node(node) => for each_child in &node.children {
                s += &each_child.join_children()
            },
            SyntaxChild::Leaf(leaf) => s += &leaf.value,
            SyntaxChild::Continuation => (),
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
    pub value: String,
    // todo: add `replaced_from`
}

impl SyntaxLeaf {
    pub fn new(value: String) -> SyntaxLeaf {
        SyntaxLeaf {
            value,
        }
    }
}
