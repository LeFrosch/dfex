mod ast;
mod lexer;

fn main() {
    let mut tree = lexer::parser("(a|b)*a(a|b)");

    tree.analyze();

    println!("{}", tree.debug_graph());
}