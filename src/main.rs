//static linkage
mod compiler; //?
mod util;

mod urmom;


//
use std::time::Instant;

//namespacing
use compiler::lexical_analysis::Lexer;
use compiler::syntax_analysis::Parser;

use compiler::lexical_analysis::Token;

use util::trie::Trie;

fn main() {

    let source = std::fs::read_to_string("test.bytelang").unwrap();
    let mut lexer = Lexer::new(&source);

    let st = Instant::now();
    let tokens = lexer.execute();
    println!("parsed in {:?}", st.elapsed().as_secs_f64());

    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    println!("{:#?}", parser.program().unwrap());

}
