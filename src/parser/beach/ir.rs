#[derive(Debug)]
pub struct Executable {
    data: Vec<Data>,
    code_sections: Vec<Code>
}

impl Executable {
    fn empty() -> Self {
        Executable {
            data: vec![],
            code_sections: vec![]
        }
    }
    pub fn from_ast(ast: super::ast::Program) -> Self {
        let mut program = Self::empty();
        todo!();
        program
    }
}

#[derive(Debug)]
pub struct Code {
    // TODO
}

#[derive(Debug)]
pub struct Data {
    pub label: String,
    pub size: usize,
    pub default: Option<Vec<u8>>
}
