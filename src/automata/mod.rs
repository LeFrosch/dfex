mod debug;
mod nfa;
mod dfa;

pub use nfa::Automata as Nfa;
pub use nfa::Node as NfaNode;
pub use dfa::Automata as Dfa;
pub use dfa::Node as DfaNode;

pub use nfa::from_tree as nfa_from_tree;
pub use dfa::from_nfa as dfa_from_nfa;