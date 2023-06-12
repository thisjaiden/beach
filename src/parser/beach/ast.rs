use crate::utils::*;

pub enum Task {
    Set { label: String, value: Value },
    Alias { label: String, points_to: String }
}

pub enum Value {
    Integer(Bigint),
    Float(Bigfloat),
    Complex(Bigcplx),
    String(String),
    Bool(bool),
}
