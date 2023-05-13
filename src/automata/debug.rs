use super::{Nfa, Dfa};

impl Nfa {
    #[allow(dead_code)]
    pub fn debug_graph(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("digraph nfa {\n");
        buffer.push_str("rankdir=LR\n");
        buffer.push_str("start[shape=none, width=0, height=0, margin=0, label=\"\"]\n");

        buffer.push_str(&format!("start -> {}\n", self.start_index));

        for node in self.states.iter() {
            buffer.push_str(&format!("{}[shape={}]\n", node.id, if node.final_state { "doublecircle" } else { "circle" }));

            for (char, targets) in node.transitions.iter() {
                for target in targets.iter() {
                    buffer.push_str(&format!("{} -> {} [label=\"{}\"]\n", node.id, target, char));   
                }
            }
        }

        buffer.push_str("}");
        buffer
    }
}


impl Dfa {
    #[allow(dead_code)]
    pub fn debug_graph(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("digraph dfa {\n");
        buffer.push_str("rankdir=LR\n");
        buffer.push_str("start[shape=none, width=0, height=0, margin=0, label=\"\"]\n");

        buffer.push_str(&format!("start -> {}\n", self.start_index));

        for node in self.states.iter() {
            buffer.push_str(&format!("{}[shape={}]\n", node.id, if node.final_state { "doublecircle" } else { "circle" }));

            for (char, target) in node.transitions.iter() {
                buffer.push_str(&format!("{} -> {} [label=\"{}\"]\n", node.id, target, char));
            }
        }

        buffer.push_str("}");
        buffer
    }
}
