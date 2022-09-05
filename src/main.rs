use anyhow::{anyhow, Result};

fn main() {}

enum Expression {
    Value(f64),

    Variable(char),

    Plus {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

struct Lexer {
    input: Vec<char>,
    index: usize, // For error reporting
}

impl Lexer {
    fn new(s: String) -> Self {
        let input: Vec<char> = s.chars().rev().collect();
        let index = 0;

        Lexer { input, index }
    }

    fn has_more(&self) -> bool {
        self.input.len() > 0
    }

    fn take_char(&mut self) -> Option<char> {
        self.input.pop()
    }

    fn peek_char(&self) -> Option<char> {
        self.input.last().copied()
    }

    fn make_tree(&mut self) -> Result<Expression> {
        while self.has_more() {
            let mut is_constant = false;
            let mut const_str = String::new();

            loop {}
        }
        todo!()
    }
}
