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
    node_id: usize,
}

impl<I : Iterator<Item=Token>> Parser<I> {
    pub fn new(scanner: I) -> Self {
        Self { scanner, stack: Vec::new(), output: Vec::new(), node_id: 0 }
    }

    fn push_out(&mut self, token: Token) {
        let node = match token {
            Token::Literal(char) => {
                let node = ast::Node::new_literal(char, self.node_id);
                self.node_id += 1;

                node
            },
            Token::Klenne => ast::Node::new_kleene(self.output.pop().unwrap()),
            Token::Alternation => ast::Node::new_alternation(self.output.pop().unwrap(), self.output.pop().unwrap()),
            Token::Concatenation => ast::Node::new_concatenation(self.output.pop().unwrap(), self.output.pop().unwrap()),
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