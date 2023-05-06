mod ast;
mod lexer;

fn main() {
    let mut tree = lexer::parse("(a|b)*a(a|b)");

    tree.analyze();

    println!("{}", tree.debug_graph());
}