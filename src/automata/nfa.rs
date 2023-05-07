use std::collections::HashMap;
use crate::ast;

use super::MultiId;

pub struct Node {
    pub id: usize,
    pub final_state: bool,
    pub transitions: HashMap<char, MultiId>
}

pub struct Automata {
    pub start_state: usize,
    pub states: Vec<Node>,
}

impl From<&ast::Tree> for Automata {
    fn from(tree: &ast::Tree) -> Self {
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
        states.push(node_from_root(literals.as_slice(), root_metadata.first.as_slice(), root_id, root_metadata.empty));

        Self { states, start_state: root_id }
    }
}

fn node_from_root(literals: &[&ast::LiteralNode], first: &[usize], id: usize, final_state: bool) -> Node {
    let mut transitions:  HashMap<char, MultiId> = HashMap::new();

    for i in first {
        transitions.entry(literals[*i].character).or_default().insert(*i);
    }

    Node { transitions, id, final_state }
}

fn node_from_literal(literals: &[&ast::LiteralNode], id: usize, final_state: bool) -> Node {
    let mut transitions:  HashMap<char, MultiId> = HashMap::new();

    let literal = &literals[id];
    for i in literal.metadata.next.iter() {
        transitions.entry(literals[*i].character).or_default().insert(*i);
    }

    Node { transitions, id, final_state }
}

impl Automata {
    #[allow(dead_code)]
    pub fn debug_graph(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("digraph automata {\n");
        buffer.push_str("rankdir=LR\n");
        buffer.push_str("start[shape=none, width=0, height=0, margin=0, label=\"\"]\n");

        buffer.push_str(&format!("start -> {}\n", self.start_state));

        for node in self.states.iter() {
            buffer.push_str(&format!("{}[shape={}]\n", node.id, if node.final_state { "doublecircle" } else { "circle" }));

            for (char, targets) in node.transitions.iter() {
                for target in targets.into_iter() {
                    buffer.push_str(&format!("{} -> {} [label=\"{}\"]\n", node.id, target, char));
                }
            }
        }

        buffer.push_str("}");
        buffer
    }
}