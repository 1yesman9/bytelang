use std::iter::IntoIterator;

use crate::util::peek::Peek;
use super::lexical_analysis::Token;
use super::syntax_analysis::ast::{Program, Statement, Assignment};

//mod
pub mod ast;

pub struct Parser<'a> {
    iter: Peek<3, <Vec<Token<'a>> as IntoIterator>::IntoIter>
}

impl<'a> Parser<'a> {
    fn expect(&mut self, target: Token<'a>) -> Result<Token<'a>, String> {
        let lookahead = self.iter.peek(0).ok_or_else(|| format!("expected {:?}, found EOF", target))?;

        if std::mem::discriminant(lookahead) == std::mem::discriminant(&target) {
            Ok(self.iter.next().unwrap())
        } else {
            Err(format!("expected {:?}, found {:?}", target, lookahead))
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

        //everything after is required
        let name = self.expect(Token::Identifier(""))?;
        self.expect(Token::Eq)?;
        let number = self.expect(Token::Number(""))?;
        Ok(Some(Assignment::new(name.inner_string().unwrap(), number.inner_string().unwrap())))
    }


    pub fn program(&mut self) -> Result<Option<Program<'a>>, String> {
        let mut statements = Vec::new();

        //this format is easy to generalize with macros
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

