use std::collections::HashMap;
use crate::util::trie::Trie;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    //
    Identifier(&'a str), // variable names, function names, etc.
    Number(&'a str),     // integer literals, float literals, etc.
    String(&'a str),     // string literals, character literals, etc.

    //Keywords
    Local,
    True,
    False,

    //Symbols
    Comma,
    DoubleEq,
    Eq,
    DoubleBar
}

impl<'a> Token<'a> {
    pub fn get_keyword(identifier: &'a str) -> Option<Token<'a>> {
        match identifier {
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "local" => Some(Token::Local),
            _ => None,
        }
    }

    pub fn get_symbol_trie() -> Trie<char, Option<Token<'a>>> {
        (&[("==", Token::DoubleEq), ("=", Token::Eq), ("||", Token::DoubleBar)][..]).into()
    }

    pub fn unwrap_ident(self) -> Option<&'a str> {
        match self {
            Token::Identifier(ident) => Some(ident),
            _ => None
        }
    }

    pub fn unwrap_number(self) -> Option<&'a str> {
        match self {
            Token::Number(ident) => Some(ident),
            _ => None
        }
    }
}
