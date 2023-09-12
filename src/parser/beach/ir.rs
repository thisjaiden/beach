use crate::parser::beach::ast::Definition;

#[derive(Debug)]
pub struct Executable {
    platform_requirements: Vec<String>,
    data: Vec<Data>,
    code_sections: Vec<Code>
}

impl Executable {
    fn empty() -> Self {
        Executable {
            platform_requirements: vec![],
            data: vec![],
            code_sections: vec![]
        }
    }
    pub fn from_ast(ast: super::ast::Program) -> Self {
        let mut program = Self::empty();
        for def in ast.definitions {
            if let Definition::System { label } = def {
                program.platform_requirements.push(label.clone());
            }
        }
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
