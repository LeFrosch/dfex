use super::Nfa;
use std::collections::{HashMap, BTreeSet};

pub struct Node {
    pub(super) id: usize,
    pub(super) final_state: bool,
    pub(super) transitions: HashMap<char, usize>,

    nfa_states: BTreeSet<usize>,
}

impl Node {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn is_final_state(&self) -> bool {
        self.final_state
    }

    pub fn iter(&self) -> impl Iterator<Item = (&char, &usize)> {
        self.transitions.iter()
    }
}

pub struct Automata {
    pub(super) start_index: usize,
    pub(super) states: Vec<Node>,
}

impl Automata {
    pub fn get_start_state(&self) -> &Node {
        &self.states[self.start_index]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.states.iter()
    }
}

pub fn from_nfa(nfa: &Nfa) -> Automata {
    let mut states: Vec<Node> = Vec::new();
    let mut frontier: Vec<usize> = Vec::new();

    let start = Node {
        id: 0,
        nfa_states: BTreeSet::from([nfa.start_index]),
        final_state: nfa.states[nfa.start_index].final_state,
        transitions: HashMap::new(),
    };

    states.push(start);
    frontier.push(0);

    while let Some(id) = frontier.pop() {
        let node = &mut states[id];

        let mut nfa_transitions: HashMap<char, BTreeSet<usize>> = HashMap::new();
        let mut dfa_transitions: HashMap<char, usize> = HashMap::new();

        for state in node.nfa_states.iter() {
            let nfa_node = &nfa.states[*state];

            for (char, target) in nfa_node.transitions.iter() {
                nfa_transitions.entry(*char).or_default().extend(target)
            }
        }

        for (key, nfa_states) in nfa_transitions {
            if let Some(state) = states.iter().find(|x| x.nfa_states == nfa_states) {
                dfa_transitions.insert(key, state.id);
            } else {
                let node = Node {
                    id: states.len(),
                    final_state: nfa_states.iter().any(|s| nfa.states[*s].final_state),
                    transitions: HashMap::new(),
                    nfa_states,
                };

                dfa_transitions.insert(key, node.id);

                states.push(node);
                frontier.push(states.len() - 1);
            }
        }

        states[id].transitions = dfa_transitions;
    }

    Automata { start_index: 0, states }
}

