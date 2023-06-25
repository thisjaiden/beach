use crate::utils::*;

pub struct Program {
    definitions: Vec<Definition>,
    global_tasks: Vec<Task>,
    main_tasks: Vec<Task>,
}

impl Program {
    pub fn from_lst(lst: super::lst::SyntaxRoot) -> Program {
        todo!()
    }
}

pub enum Definition {
    Alias { label: String, points_to: String },
    GlobalConstant { label: String, value: Value },
    Function { label: String, tasks: Vec<Task> },
}

pub enum Task {
    Set { label: String, type_: Option<String>, value: Value },
    Call { label: String, arguments: Vec<Value> },
}

pub enum Value {
    Integer(Bigint),
    Float(Bigfloat),
    Complex(Bigcplx),
    String(String),
    Bool(bool),
    Label(String),
}
