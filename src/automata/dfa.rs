use super::nfa;
use super::MultiId;
use std::collections::HashMap;
use std::fmt::format;

pub struct Node {
    id: MultiId,
    final_state: bool,
    transitions: HashMap<char, MultiId>,
}

pub struct Automata {
    pub start_state: usize,
    pub states: Vec<Node>,
}

impl From<&nfa::Automata> for Automata {
    fn from(nfa: &nfa::Automata) -> Self {
        let mut states: Vec<Node> = Vec::new();
        let mut frontier: Vec<usize> = Vec::new();

        let start = Node {
            id: MultiId::new(nfa.start_state),
            final_state: nfa.states[nfa.start_state].final_state,
            transitions: HashMap::new(),
        };

        states.push(start);
        frontier.push(0);

        while let Some(id) = frontier.pop() {
            let node = &mut states[id];
            let mut transitions: HashMap<char, MultiId> = HashMap::new();

            for state in &node.id {
                let nfa_node = &nfa.states[*state];

                for (char, target) in nfa_node.transitions.iter() {
                    transitions.entry(*char).or_default().extend(target)
                }
            }

            for (_, target) in transitions.iter() {
                if states.iter().any(|x| &x.id == target) {
                    continue;
                }

                let node = Node {
                    id: target.clone(),
                    final_state: target.into_iter().any(|s| nfa.states[*s].final_state),
                    transitions: HashMap::new(),
                };

                states.push(node);
                frontier.push(states.len() - 1);
            }
            
            states[id].transitions = transitions;
        }

        Self {
            start_state: 0,
            states,
        }
    }
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
            buffer.push_str(&format!("{}[shape={}]\n", node_id(&node.id), if node.final_state { "doublecircle" } else { "circle" }));

            for (char, target) in node.transitions.iter() {
                buffer.push_str(&format!("{} -> {} [label=\"{}\"]\n", node_id(&node.id), node_id(target), char));
            }
        }

        buffer.push_str("}");
        buffer
    }
}

fn node_id(node: &MultiId) -> String {
    let mut buffer = String::new();

    for i in node {
        buffer.push_str(&format!("{}", *i));
    }

    buffer
}

