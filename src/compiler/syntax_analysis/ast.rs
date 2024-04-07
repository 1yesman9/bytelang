#[derive(Debug)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>
}

impl<'a> Program<'a> {
    pub fn new(statements: Vec<Statement<'a>>) -> Self {
        Self { statements }
    }
}

#[derive(Debug)]

pub enum Statement<'a> {
    Assignment(Assignment<'a>)
}

#[derive(Debug)]
pub struct Assignment<'a> {
    name: &'a str,
    value: &'a str
}

impl<'a> Assignment<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}

//urmom gayyy
