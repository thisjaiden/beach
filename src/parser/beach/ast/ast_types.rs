use std::iter::Peekable;

use crate::utils::*;
use crate::parser::beach::ast::user_token_format::Symbol;

/// Describes a generic definition of something important to the program.
#[derive(Debug)]
pub enum Definition {
    /// A standard library import from beach/std/core/*
    System { label: String },
    /// A project file import from the root file
    File { label: String },
    /// An external library from the TODO(beach package manager)
    Library { name: String, version: String },
    Alias { from: String, to: String, export: bool },
    GlobalConstant { label: String, value: Value },
    Function(Function),
    Type {
        name: String,
        data: Vec<(String, TypeIdentity)>,
        methods: Vec<Function>
    },
    Trait { name: String, methods: Vec<Function> }
}

/// Describes a task to be completed by the program at runtime.
#[derive(Debug)]
pub enum Task {
    Set { label: String, type_: Option<String>, value: Value },
    Call { function_identifier: String, arguments: Vec<Value> },
    Evaluate { label: String, task: Evaluatable },
    FreeEvaluated { label: String },
    ExitBlock,
}

#[derive(Debug)]
pub enum Evaluatable {
    Call { label: String, arguments: Vec<Value> },
    Math { a: Value, b: Value },
    Value { value: Value }
}

impl Evaluatable {
    pub fn from_symbols<'a, I>(syms: &mut Peekable<I>, end: Symbol) -> Self
    where
        I: Iterator<Item = &'a Symbol> {
        match syms.next().expect("Called with null sym, should be impossible") {
            Symbol::String(symstr) => {
                return Self::Value { value: Value::String(symstr.clone()) }
            }
            Symbol::Integer(symint) => {
                return Self::Value { value: Value::Integer(symint.clone()) }
            }
            sym => todo!("evaluatble from_symbols sym ({:?})", sym)
        }
        todo!();
    }
}

// TODO: cannot represent non-primitive types directly, only through labels.
#[derive(Debug, Clone)]
pub enum Value {
    Integer(Bigint),
    Float(Bigfloat),
    Complex(Bigcplx),
    String(String),
    Bool(bool),
    Label(String),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub returns: TypeIdentity,
    pub code: Vec<Task>
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub arg_type: TypeIdentity,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    ArchUnsigned, // usize
    ArchSigned, // isize
    U8, // u8
    I8, // i8
}

#[derive(Debug, Clone)]
pub enum TypeIdentity {
    Primitive(PrimitiveType),
    Trait(String),
    Enum(String),
    Structured(String),
}
