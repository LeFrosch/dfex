use std::{iter, slice};

pub struct Metadata {
    pub empty: bool,
    pub first: Vec<usize>,
    pub next: Vec<usize>,
    pub last: Vec<usize>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            empty: false,
            first: vec![],
            next: vec![],
            last: vec![],
        }
    }
}

pub struct AlternationNode {
    pub metadata: Metadata,
    pub children: Vec<Node>,
}

pub struct ConcatenationNode {
    pub metadata: Metadata,
    pub children: Vec<Node>,
}

pub struct KleeneNode {
    pub metadata: Metadata,
    pub child: Box<Node>,
}

pub struct LiteralNode {
    pub metadata: Metadata,
    pub id: usize,
    pub character: char,
}

pub enum Node {
    Alternation(AlternationNode),
    Concatenation(ConcatenationNode),
    Kleene(KleeneNode),
    Literal(LiteralNode),
}

impl Node {
    pub fn metadata(&self) -> &Metadata {
        match self {
            Node::Alternation(node) => &node.metadata,
            Node::Concatenation(node) => &node.metadata,
            Node::Kleene(node) => &node.metadata,
            Node::Literal(node) => &node.metadata,
        }
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        match self {
            Node::Alternation(node) => &mut node.metadata,
            Node::Concatenation(node) => &mut node.metadata,
            Node::Kleene(node) => &mut node.metadata,
            Node::Literal(node) => &mut node.metadata,
        }
    }

    pub fn children(&self) -> &[Node] {
        match self {
            Node::Alternation(node) => node.children.as_slice(),
            Node::Concatenation(node) => node.children.as_slice(),
            Node::Kleene(node) => slice::from_ref(&node.child),
            Node::Literal(_) => &[],
        }
    }

    pub fn children_mut(&mut self) -> &mut [Node] {
        match self {
            Node::Alternation(node) => node.children.as_mut_slice(),
            Node::Concatenation(node) => node.children.as_mut_slice(),
            Node::Kleene(node) => slice::from_mut(&mut node.child),
            Node::Literal(_) => &mut [],
        }
    }

    pub fn iter_pre<F>(&self, f: &mut F) where F: FnMut(&Node) {
        for child in self.children() {
            child.iter_pre(f);
        }
        f(self);
    }

    pub fn iter_pre_mut<F>(&mut self, f: &mut F) where F: FnMut(&mut Node) {
        for child in self.children_mut() {
            child.iter_pre_mut(f);
        }
        f(self);
    }

    pub fn iter_post_mut<F>(&mut self, f: &mut F) where F: FnMut(&mut Node) {
        f(self);
        for child in self.children_mut() {
            child.iter_post_mut(f);
        }
    }
}