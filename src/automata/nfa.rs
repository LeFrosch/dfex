use crate::ast;
use std::collections::{HashMap, BTreeSet};

pub struct Node {
    pub(super) id: usize,
    pub(super) final_state: bool,
    pub(super) transitions: HashMap<char, BTreeSet<usize>>,
}

pub struct Automata {
    pub(super) start_index: usize,
    pub(super) states: Vec<Node>,
}

pub fn from_tree(tree: &ast::Tree) -> Automata {
    let mut literals = Vec::new();
    tree.iter_pre(|node| {
        if let ast::Node::Literal(literal) = node {
            literals.insert(literal.id, literal);
        }
    });

    let root_metadata = tree.root.metadata();

    let mut states: Vec<Node> = (0..literals.len())
        .map(|i| node_from_literal(literals.as_slice(), i, root_metadata.last.contains(&i)))
        .collect();

    let root_id = states.len();
    states.push(node_from_root(
        literals.as_slice(),
        root_metadata.first.as_slice(),
        root_id,
        root_metadata.empty,
    ));

    Automata { states, start_index: root_id }
}
fn node_from_root(
    literals: &[&ast::LiteralNode],
    first: &[usize],
    id: usize,
    final_state: bool,
) -> Node {
    let mut transitions: HashMap<char, BTreeSet<usize>> = HashMap::new();

    for i in first {
        transitions.entry(literals[*i].character).or_default().insert(*i);
    }

    Node { transitions, id, final_state }
}

fn node_from_literal(literals: &[&ast::LiteralNode], id: usize, final_state: bool) -> Node {
    let mut transitions: HashMap<char, BTreeSet<usize>> = HashMap::new();

    let literal = &literals[id];
    for i in literal.metadata.next.iter() {
        transitions.entry(literals[*i].character).or_default().insert(*i);
    }

    Node { transitions, id, final_state }
}

