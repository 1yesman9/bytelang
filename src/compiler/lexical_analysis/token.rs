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

    pub fn inner_string(self) -> Option<&'a str> {
        Some(match self {
            Token::Identifier(ident) => ident,
            Token::Number(number) => number,
            Token::String(string) => string,
            _ => return None
        })
    }
}
