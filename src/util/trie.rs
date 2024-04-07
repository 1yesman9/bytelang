use std::collections::HashMap;
use crate::compiler::lexical_analysis::Token;

#[derive(Debug)]
pub struct Trie<I, V> {
    pub value: V,
    children: HashMap<I, Trie<I, V>>
}

impl<I: Eq + PartialEq + std::hash::Hash, V> Trie<I, V> {
    pub fn new(value: V) -> Self {
        Self { value, children: HashMap::new() }
    }
    pub fn match_child(&self, index: &I) -> Option<&Trie<I, V>> {
        self.children.get(index)
    }
}

impl<'a> std::convert::From<&[(&str, Token<'a>)]> for Trie<char, Option<Token<'a>>> {
    fn from(value: &[(&str, Token<'a>)]) -> Self {
        let mut root = Trie::new(None);

        for (string, token) in value {
            let mut trie = &mut root;

            //loop through each character in string
            for (index, chr) in string.chars().enumerate() {
                trie = if trie.children.contains_key(&chr) {
                    //if the character already has a node at this location, go into it
                    let child = trie.children.get_mut(&chr).unwrap();
                    if index == string.len() - 1 { child.value = Some(*token); }
                    child

                } else {
                    //otherwise, insert the new node, and start working with it
                    trie.children.insert(chr, Trie::new(if index == string.len()-1 { Some(*token)} else { None }));
                    trie.children.get_mut(&chr).unwrap()
                };
            }
        }

        root
    }
}
