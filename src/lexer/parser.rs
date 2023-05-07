use crate::ast;

use super::scanner::Token;
use super::{Result, Error};

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

    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    pub fn parse(&mut self) -> Result<ast::Node> {
        while let Some(token) = self.scanner.next() {
            match token {
                Token::Literal(_) => self.push_out(token)?,
                Token::RParen => self.find_matching_lparen()?,
                Token::LParen => self.stack.push(token),
                _ => self.push_operator(token)?,
            }
        }

        while let Some(token) = self.stack.pop() {
            self.push_out(token)?;
        }

        self.output.pop().ok_or(Error)
    }

    fn push_out(&mut self, token: Token) -> Result<()> {
        let node = match token {
            Token::Literal(char) => {
                let node = ast::Node::new_literal(char, self.node_id);
                self.node_id += 1;

                node
            },
            Token::Klenne => ast::Node::new_kleene(self.output.pop().ok_or(Error)?),
            Token::Alternation => ast::Node::new_alternation(self.output.pop().ok_or(Error)?, self.output.pop().ok_or(Error)?),
            Token::Concatenation => ast::Node::new_concatenation(self.output.pop().ok_or(Error)?, self.output.pop().ok_or(Error)?),
            Token::LParen | Token::RParen | Token::Invalid(_) => return Err(Error),
        };

        self.output.push(node);

        Ok(())
    }

    fn push_operator(&mut self, token: Token) -> Result<()> {
        while let Some(last) = self.stack.last() {
            if token.precedence() > last.precedence() {
                break
            }

            let last = self.stack.pop().ok_or(Error)?;
            self.push_out(last)?;
        }

        self.stack.push(token);

        Ok(())
    }

    fn find_matching_lparen(&mut self) -> Result<()> {
        while let Some(token) = self.stack.pop() {
            if token == Token::LParen {
                return Ok(());
            }

            self.push_out(token)?;
        }

        Err(Error)
    }
}