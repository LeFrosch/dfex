mod ast;
mod automata;
mod lexer;

fn main() {
    let mut tree = lexer::parse("(a|b)*a(a|b)").unwrap();
    tree.analyze();

    let nfa = automata::Nfa::from(&tree);
    let dfa = automata::Dfa::from(&nfa);

    println!("{}", dfa.debug_graph());
}

