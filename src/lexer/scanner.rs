use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq)]
pub enum Token {
    Literal(char),
    Klenne,
    Alternation,
    Concatenation,
    LParen,
    RParen,
    Invalid(char),
}

pub struct Scanner<'a> {
    chars: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { chars: text.chars() }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(map_char)
    }
}

fn map_char(char: char) -> Token {
    match char {
        '(' => Token::LParen,
        ')' => Token::RParen,
        '|' => Token::Alternation,
        '*' => Token::Klenne,
        'a'..='z' | 'A'..='Z' | '0'..='9' => Token::Literal(char),
        c => Token::Invalid(c),
    }
}

pub struct AutoConcatenation<'a> {
    scanner: Peekable<Scanner<'a>>,
    insert_concatenation: bool,
}

impl<'a> AutoConcatenation<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self { scanner: scanner.peekable(), insert_concatenation: false }
    }
}

impl<'a> Iterator for AutoConcatenation<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.insert_concatenation {
            self.insert_concatenation = false;
            return Some(Token::Concatenation);
        }

        let token = self.scanner.next()?;

        match token {
            Token::Literal(_) | Token::Klenne | Token::RParen => match self.scanner.peek() {
                Some(Token::Literal(_)) | Some(Token::LParen) => self.insert_concatenation = true,
                _ => {}
            },
            _ => {}
        }

        Some(token)
    }
}
