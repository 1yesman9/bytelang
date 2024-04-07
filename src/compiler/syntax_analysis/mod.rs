use std::iter::IntoIterator;

use super::lexical_analysis::Token;

use crate::util::peek::Peek;

use super::syntax_analysis::ast::{Program, Statement, Assignment};

//mod
pub mod ast;

pub struct Parser<'a> {
    iter: Peek<3, <Vec<Token<'a>> as IntoIterator>::IntoIter>
}

impl<'a> Parser<'a> {
    fn expect(&mut self, target: Token<'a>) -> Result<Token<'a>, String> {
        let exists = self.iter.peek(0)
            .is_some_and(|token| std::mem::discriminant(token) == std::mem::discriminant(&target));

        if exists {
            //report if there's no token left,
            //report if we got the wrong thing
            self.iter.next().ok_or(format!("expected {:?}", target))
        } else {
            Err(format!("expected {:?}", target))
        }
    }

    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            iter: Peek::from(tokens.into_iter())
        }
    }

    pub fn assignment(&mut self) -> Result<Option<Assignment<'a>>, String> {
        //if we don't encounter local immediately, no assignment is found
        if let Err(_) = self.expect(Token::Local) { return Ok(None) }
        let name = self.expect(Token::Identifier(""))?;
        self.expect(Token::Eq)?;
        let number = self.expect(Token::Number(""))?;
        Ok(Some(Assignment::new(name.unwrap_ident().unwrap(), number.unwrap_number().unwrap())))
    }


    pub fn program(&mut self) -> Result<Option<Program<'a>>, String> {
        let mut statements = Vec::new();

        loop {
            //match this kind of statement
            if let Some(assignment) = self.assignment()? {
                statements.push(Statement::Assignment(assignment));
                continue
            }

            //match another kind of statement

            break
        }

        Ok(Some(Program::new(statements)))
    }
}

