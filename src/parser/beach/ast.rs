use std::iter::Peekable;

use crate::{utils::*, parser::beach::lst::{Symbol, keywords::Keyword}};

#[derive(Debug)]
pub struct Program {
    definitions: Vec<Definition>,
    global_tasks: Vec<Task>,
    main_tasks: Vec<Task>,
}

impl Program {
    pub fn from_lst(lst: super::lst::SyntaxRoot) -> Program {
        let mut program = Program { definitions: vec![], global_tasks: vec![], main_tasks: vec![] };
        let mut syms = lst.symbols.iter().peekable();
        // TODO: split this into several functions for basic context and structure
        while syms.peek().is_some() {
            match syms.next().unwrap() {
                Symbol::Comment(_) | Symbol::Comments(_) => {}
                Symbol::Keyword(kwrd) => {
                    match kwrd {
                        Keyword::needs => {
                            // expects Label(_), PhraseEnd
                            if let Some(&&Symbol::Label(ref label)) = syms.peek() {
                                // Label(_) found
                                syms.next();
                                // check for PhraseEnd
                                if let Some(&&Symbol::PhraseEnd) = syms.peek() {
                                    // PhraseEnd found! Statement complete!
                                    syms.next();
                                    program.definitions.push(Definition::Needs { label: label.clone() });
                                }
                                else {
                                    panic!("Expected `;` following `needs {}`. ({{TODO: ANNOTATIONS}})", label);
                                }
                            }
                        }
                        Keyword::wants => {
                            // expects Label(_), PhraseEnd
                            if let Some(&&Symbol::Label(ref label)) = syms.peek() {
                                // Label(_) found
                                syms.next();
                                // check for PhraseEnd
                                if let Some(&&Symbol::PhraseEnd) = syms.peek() {
                                    // PhraseEnd found! Statement complete!
                                    syms.next();
                                    program.definitions.push(Definition::Wants { label: label.clone() });
                                }
                                else {
                                    panic!("Expected `;` following `wants {}`. ({{TODO: ANNOTATIONS}})", label);
                                }
                            }
                        }
                        Keyword::main => {
                            if Some(&&Symbol::OpenBrace) == syms.peek() {
                                syms.next();
                                program.main_scope(&mut syms);
                            }
                            else {
                                panic!("Expected `{{` following keyword `main`. ({{TODO: ANNOTATIONS}})");
                            }
                        }
                        _ => todo!()
                    }
                }
                Symbol::Label(l) => {
                    // Running a function if we find OpenParenthesis, PhraseEnd
                    if let Some(&&Symbol::OpenParenthesis) = syms.peek() {
                        todo!("NOT DONE");
                        // OpenParenthesis found
                        syms.next();
                        // we should expected a comma seperated list of `Value`s now, ending with CloseParenthesis, PhraseEnd
                        let mut arguments = vec![];
                        while syms.peek() != Some(&&Symbol::Closeparenthesis) {
                            // TODO
                            // ...
                        }
                        program.global_tasks.push(Task::Call { label: l.to_string(), arguments });
                    }
                    // Creating an alias if we find Alias, PhraseEnd
                    if let Some(&&Symbol::Alias) = syms.peek() {
                        // Alias found
                        syms.next();
                        // check for Label(_)
                        if let Some(&&Symbol::Label(ref outlabel)) = syms.peek() {
                            syms.next();
                            // while we find the Module symbol, dump the following Label(_) into a vec
                            let mut out_lab_with_refs = vec![];
                            out_lab_with_refs.push(outlabel.to_string());
                            while Some(&&Symbol::Module) == syms.peek() {
                                syms.next();
                                if let Some(&&Symbol::Label(ref suboutlabel)) = syms.peek() {
                                    syms.next();
                                    out_lab_with_refs.push(suboutlabel.to_string());
                                }
                                else {
                                    panic!("Expected a label following a module seperator `~` in an alias statement. ({{TODO ANNOTATIONS}})");
                                }
                            }
                            // check for PhraseEnd
                            if let Some(&&Symbol::PhraseEnd) = syms.peek() {
                                // PhraseEnd found! Statement complete!
                                syms.next();
                                program.definitions.push(Definition::Alias { label: l.clone(), points_to: outlabel.to_string() });
                            }
                            else {
                                panic!("Expected `;` following an alias statement. ({{TODO: ANNOTATIONS}})");
                            }
                        }
                        else {
                            panic!("Expected a label following the alias operator `=>`. ({{TODO: ANNOTATIONS}})");
                        }
                    }
                }
                k => todo!("{:?}", k)
            }
        }
        todo!()
    }
    fn main_scope<'a, I>(&mut self, syms: &mut Peekable<I>)
    where
        I: Iterator<Item = &'a Symbol> {
        while syms.peek().is_some() {
            match syms.next().unwrap() {
                Symbol::Comment(_) | Symbol::Comments(_) => {},
                _ => { todo!() }
            }
        }
    }
}

#[derive(Debug)]
pub enum Definition {
    Wants { label: String },
    Needs { label: String },
    Alias { label: String, points_to: String },
    GlobalConstant { label: String, value: Value },
    Function { label: String, tasks: Vec<Task> },
}

#[derive(Debug)]
pub enum Task {
    Set { label: String, type_: Option<String>, value: Value },
    Call { label: String, arguments: Vec<Value> },
}

#[derive(Debug)]
pub enum Evaluatable {
    Call { label: String, arguments: Vec<Value> },
    Math { a: Value, b: Value }
}

#[derive(Debug)]
pub enum Value {
    Integer(Bigint),
    Float(Bigfloat),
    Complex(Bigcplx),
    String(String),
    Bool(bool),
    Label(Vec<String>),
}
