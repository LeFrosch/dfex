use crate::ast;

mod scanner;
mod parser;

pub fn parse(text: &str) -> ast::Tree {
    let scanner = scanner::AutoConcatenation::new(scanner::Scanner::new(text));
    let mut parser = parser::Parser::new(scanner);

    let root = parser.parse();

    ast::Tree { root }
}

#[cfg(test)]
mod tests {
    use super::ast;
    use super::parse;

    fn to_expr(tree: &ast::Tree) -> String {
        let mut buffer = String::new();

        tree.iter_pre(|node| {
            let char = match node {
                ast::Node::Alternation(_) => '|',
                ast::Node::Concatenation(_) => '.',
                ast::Node::Kleene(_) => '*',
                ast::Node::Literal(node) => node.character,
            };

            buffer.push(char);
        });

        buffer
    }

    macro_rules! parse_tests {
        ($($input:literal -> $output:literal),+) => {
            #[test]
            fn parse_tests() {
                $({
                    let tree = parse($input);
                    let text = to_expr(&tree);

                    assert_eq!(text, $output, "rpn of \"{}\" should be \"{}\"", $input, $output);
                })+
            }
        };
    }

    parse_tests!(
        "a*b" -> "a*b.",
        "a*b*" -> "a*b*.",
        "a|b" -> "ab|",
        "a|b|c" -> "ab|c|",
        "ab" -> "ab.",
        "abc" -> "ab.c.",
        "ab|c" -> "ab.c|",
        "a(b|c)" -> "abc|.",
        "a|bc" -> "abc.|",
        "ab*" -> "ab*.",
        "(ab)*" -> "ab.*",
        "(a|b)*a(a|b)" -> "ab|*a.ab|."
    );
}