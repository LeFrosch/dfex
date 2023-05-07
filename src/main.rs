mod ast;
mod lexer;

fn main() {
    let mut tree = lexer::parse("(a|b)*a(a|b)").unwrap();

    tree.analyze();

    println!("{}", tree.debug_graph());
}