use proc_macro::TokenStream;
use syn::parse_macro_input;

mod ast;
mod automata;
mod lexer;
mod gen;

#[proc_macro]
pub fn dfa(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as gen::Input);

    let mut tree = lexer::parse(&input.regex).unwrap();
    tree.analyze();

    let nfa = automata::nfa_from_tree(&tree);
    let dfa = automata::dfa_from_nfa(&nfa);

    gen::matcher(&input.ident, &dfa)
}

