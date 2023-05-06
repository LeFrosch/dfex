use crate::ast;

use super::scanner::Token;

impl Token {
    fn precedence(&self) -> u8 {
        match self {
            Token::Klenne => 3,
            Token::Concatenation => 2,
            Token::Alternation => 1,
            _ => 0,
        }
    }
}

pub struct Parser<I : Iterator<Item=Token>> {
    scanner: I,
    stack: Vec<Token>,
    output: Vec<ast::Node>,
}

impl<I : Iterator<Item=Token>> Parser<I> {
    pub fn new(scanner: I) -> Self {
        Self { scanner, stack: Vec::new(), output: Vec::new() }
    }

    fn push_out(&mut self, token: Token) {
        let node = match token {
            Token::Literal(char) => ast::Node::new_literal(char, 0),
            Token::Klenne => ast::Node::new_kleene(self.output.pop().unwrap()),
            Token::Alternation => ast::Node::new_alternation(vec![self.output.pop().unwrap(), self.output.pop().unwrap()]),
            Token::Concatenation => ast::Node::new_concatenation(vec![self.output.pop().unwrap(), self.output.pop().unwrap()]),
            Token::LParen | Token::RParen => panic!(),
        };

        self.output.push(node);
    }

    fn push_operator(&mut self, token: Token) {
        while let Some(last) = self.stack.last() {
            if token.precedence() > last.precedence() {
                break
            }

            let last = self.stack.pop().unwrap();
            self.push_out(last);
        }

        self.stack.push(token);
    }

    pub fn parse(&mut self) -> ast::Node {
        while let Some(token) = self.scanner.next() {
            match token {
                Token::Literal(_) => self.push_out(token),
                Token::RParen => {
                    while let Some(token) = self.stack.pop() {
                        if token == Token::LParen {
                            break
                        }

                        self.push_out(token);
                    }
                },
                Token::LParen => self.stack.push(token),
                _ => self.push_operator(token),
            }
        }

        while let Some(token) = self.stack.pop() {
            self.push_out(token);
        }

        self.output.pop().unwrap()
    }
}


/*
#[cfg(test)]
mod tests {
    use std::iter::Peekable;
    use super::Token;
    use crate::lexer::scanner;
    use super::rpn;

    fn to_expr(tokens: &Vec<Token>) -> String {
        let mut buffer = String::new();

        for token in tokens {
            match token {
                Token::Literal(c) => buffer.push(*c),
                Token::Klenne => buffer.push('*'),
                Token::Alternation => buffer.push('|'),
                Token::Concatenation => buffer.push('.'),
                Token::LParen => buffer.push('('),
                Token::RParen => buffer.push(')'),
            }
        }

        buffer
    }

    macro_rules! rpn_tests {
        ($($input:literal -> $output:literal),+) => {
            #[test]
            fn rpn_tests() {
                $({
                    let mut scanner = scanner::AutoConcatenation::new(scanner::Scanner::new($input));
                    let result = rpn(&mut scanner);

                    assert_eq!(to_expr(&result), $output, "rpn of \"{}\" should be \"{}\"", $input, $output);
                })+
            }
        };
    }

    rpn_tests!(
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
 */