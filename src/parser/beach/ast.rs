use std::{iter::Peekable, ops::Add};

use crate::{utils::*, parser::beach::lst::{Symbol, keywords::Keyword}};

#[derive(Debug)]
pub struct Program {
    pub definitions: Vec<Definition>,
    pub global_tasks: Vec<Task>,
    pub main_tasks: Vec<Task>,
    pending_file_additions: Vec<String>,
}

impl Add for Program {
    type Output = Program;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        let mut two = rhs;
        out.definitions.append(&mut two.definitions);
        out.global_tasks.append(&mut two.global_tasks);
        out.global_tasks.append(&mut two.main_tasks);
        return out;
    }
}

impl Program {
    pub fn from_lst(lst: super::lst::Syntax, prefix: Option<String>) -> Result<Program, anyhow::Error> {
        let mut program = Program {
            definitions: vec![],
            global_tasks: vec![],
            main_tasks: vec![],
            pending_file_additions: vec![]
        };
        let mut syms = lst.symbols.iter().peekable();
        program.global_scope(&mut syms)?;
        while !program.pending_file_additions.is_empty() {
            let mut glob_addition = String::new();
            for addition in &program.pending_file_additions {
                glob_addition += "\n";
                glob_addition += addition;
            }
            program.pending_file_additions.clear();
            let parsed = super::parse_string_file(glob_addition);
            let mut syms = parsed.symbols.iter().peekable();
            program.global_scope(&mut syms)?;
        }
        Ok(program)
    }
    fn global_scope<'a, I>(&mut self, syms: &mut Peekable<I>) -> Result<(), anyhow::Error>
    where
        I: Iterator<Item = &'a Symbol> {
        while syms.peek().is_some() {
            match syms.next().unwrap() {
                Symbol::Comment(_) | Symbol::Comments(_) => {}
                Symbol::Keyword(kwrd) => {
                    match kwrd {
                        Keyword::Ksystem => {
                            // expects Label(_), PhraseEnd
                            if let Some(&&Symbol::Label(ref label)) = syms.peek() {
                                // Label(_) found
                                syms.next();
                                // check for PhraseEnd
                                if let Some(&&Symbol::PhraseEnd) = syms.peek() {
                                    // PhraseEnd found! Statement complete!
                                    syms.next();
                                    self.definitions.push(Definition::System { label: label.clone() });
                                }
                                else {
                                    return Err(anyhow::Error::msg(
                                        format!("Expected `;` following `system {}`. (TODO: ANNOTATIONS)", label)
                                    ));
                                }
                            }
                        }
                        Keyword::Kmain => {
                            if Some(&&Symbol::OpenBrace) == syms.peek() {
                                syms.next();
                                self.main_scope(syms)?;
                            }
                            else {
                                return Err(anyhow::Error::msg(
                                    "Expected `{` following keyword `main`. (TODO: ANNOTATIONS)"
                                ));
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
                        while syms.peek() != Some(&&Symbol::CloseParenthesis) {
                            // TODO
                            // ...
                            todo!();
                        }
                        self.global_tasks.push(Task::Call { label: l.to_string(), arguments });
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
                                    return Err(anyhow::Error::msg(
                                        "Expected a label following a module seperator (~) in an alias statement. (TODO: ANNOTATIONS)"
                                    ));
                                }
                            }
                            // check for PhraseEnd
                            if let Some(&&Symbol::PhraseEnd) = syms.peek() {
                                // PhraseEnd found! Statement complete!
                                syms.next();
                                self.definitions.push(Definition::Alias { label: l.clone(), points_to: out_lab_with_refs.clone() });
                            }
                            else {
                                return Err(anyhow::Error::msg(
                                    "Expected `;` following an alias statement. (TODO: ANNOTATIONS)"
                                ));
                            }
                        }
                        else {
                            return Err(anyhow::Error::msg(
                                "Expected a label following the alias operator. (`=>`) (TODO: ANNOTATIONS)"
                            ));
                        }
                    }
                }
                Symbol::Compiler => {
                    self.compiler_directive(syms)?;
                }
                k => todo!("{:?}", k)
            }
        }
        Ok(())
    }
    fn compiler_directive<'a, I>(&mut self, mut syms: &mut Peekable<I>) -> Result<(), anyhow::Error>
    where
        I: Iterator<Item = &'a Symbol> {
        if let Some(&Symbol::Label(lbl)) = syms.peek() {
            match lbl.as_str() {
                "core" => {
                    // discard the "core" label
                    syms.next();
                    if syms.peek().is_none() {
                        return Err(anyhow::Error::msg(
                            "Unexpected end of file following the compiler tag `!!core`. (TODO: ANNOTATIONS)"
                        ));
                    }
                    // check that there is at least one lable following our "core" delegation
                    match syms.peek() {
                        Some(&Symbol::Label(_)) => {}, // ignore
                        _ => {
                            return Err(anyhow::Error::msg(
                                "Unexpected symbol following the compiler tag `!!core`. (TODO: ANNOTATIONS)"
                            ));
                        }
                    }
                    let mut path = install_directory();
                    path.push("std");
                    path.push("core");
                    while let Some(&Symbol::Label(lbl)) = syms.peek() {
                        // discard label
                        syms.next();
                        match syms.peek() {
                            Some(&&Symbol::Divide) => {
                                // our path continues!
                                // add to our path as a dir
                                path.push(lbl);
                                // throw away our "/" so we loop
                                syms.next();
                            }
                            Some(&&Symbol::PhraseEnd) => {
                                // our path ends here.
                                path.push(format!("{}.beach", lbl));
                            }
                            Some(_) => {
                                return Err(anyhow::Error::msg(
                                    "Unexpected symbol during compiler directive. (TODO: ANNOTATIONS)"
                                ));
                            }
                            None => {
                                return Err(anyhow::Error::msg(
                                    "Unexpected end of file during compiler directive. (TODO: ANNOTATIONS)"
                                ));
                            }
                        }
                    }
                    // check we end with a PhraseEnd like we should
                    if syms.peek() != Some(&&Symbol::PhraseEnd) {
                        return Err(anyhow::Error::msg(
                            "Expected a semicolon following a compiler directive. (TODO: ANNOTATIONS)"
                        ));
                    }
                    // TODO: do something with the path we found
                    let core_string = std::fs::read_to_string(path)?;
                    self.pending_file_additions.push(core_string);
                    // throw away PhraseEnd, and we're done!
                    syms.next();
                },
                _ => unimplemented!("unknown label {}", lbl)
            }
            Ok(())
        }
        else {
            Err(anyhow::Error::msg(
                "Expected a keyword following the compiler directive `!!` (TODO: ANNOTATIONS)"
            ))
        }
    }
    fn main_scope<'a, I>(&mut self, syms: &mut Peekable<I>) -> Result<(), anyhow::Error>
    where
        I: Iterator<Item = &'a Symbol> {
        while syms.peek().is_some() {
            match syms.next().unwrap() {
                Symbol::Comment(_) | Symbol::Comments(_) => {},
                Symbol::Label(l) => {
                    match syms.peek() {
                        Some(Symbol::OpenParenthesis) => {
                            // This is a function call!
                            // throw away open parrens
                            syms.next();
                            // we should expect a comma seperated list of `Evaluatable`s now,
                            // ending with CloseParenthesis, PhraseEnd
                            let mut eval_idx = 0;
                            let mut not_eval = vec![];
                            while syms.peek() != Some(&&Symbol::CloseParenthesis) {
                                // TODO: recursive function calls could have collisions...
                                // TODO: doc this weird shit or improve it
                                let task = Evaluatable::from_symbols(syms, Symbol::Also);
                                if let Evaluatable::Value { value } = task {
                                    not_eval.push(Some(value));
                                    // if we have another argument, throw away the comma between args.
                                    if syms.peek() == Some(&&Symbol::Also) {
                                        syms.next();
                                    }
                                    continue;
                                }
                                self.main_tasks.push(Task::Evaluate {
                                    label: format!("compiler_ast_call_eval_{}", eval_idx),
                                    task
                                });
                                not_eval.push(None);
                                eval_idx += 1;
                                // if we have another argument, throw away the comma between args.
                                if syms.peek() == Some(&&Symbol::Also) {
                                    syms.next();
                                }
                            }
                            // throw away close parrens
                            syms.next();
                            // last sym should be PhraseEnd
                            if syms.peek() != Some(&&Symbol::PhraseEnd) {
                                // if it's not, panic
                                panic!("Expected `;` following a function call. (TODO: ANNOTATIONS)");
                            }
                            // throw away PhraseEnd
                            syms.next();
                            let mut arguments: Vec<Value> = vec![];
                            for (idx, i) in not_eval.iter().enumerate() {
                                if let Some(val) = i {
                                    arguments.push(val.clone());
                                }
                                else {
                                    arguments.push(Value::Label(format!("compiler_ast_call_eval_{idx}")));
                                }
                            }
                            let mut target = l.clone();
                            for def in &self.definitions {
                                if let Definition::Alias { label: aptfr, points_to: aptto } = &def {
                                    if l == aptfr {
                                        target = aptto[0].clone();
                                    }
                                }
                            }
                            self.main_tasks.push(Task::Call { label: target.clone(), arguments });
                            for (idx, i) in not_eval.iter().enumerate() {
                                if i.is_none() {
                                    self.main_tasks.push(
                                        Task::FreeEvaluated {
                                            label: format!("compiler_ast_call_eval_{idx}")
                                        }
                                    );
                                }
                            }
                        }
                        Some(Symbol::PhraseEnd) => {
                            // This is a no-argument function call.
                            // throw away phrase end
                            syms.next();
                            // add to tasks
                            self.main_tasks.push(Task::Call {
                                label: l.clone(),
                                arguments: vec![]
                            });
                        }
                        Some(sym) => { todo!("TODO ICE Label(_) -> sym ({:?})", sym) }
                        None => { panic!("Abrupt EOF before closing the `main` segment, and before terminating a line.") }
                    }
                }
                Symbol::CloseBrace => {
                    // end of main block
                    return Ok(());
                }
                Symbol::Keyword(sym_kywrd) => {
                    match sym_kywrd {
                        Keyword::Kreturn => {
                            // return from main
                            // TODO: may not work inside deeper blocks
                            // should be followed by a PhraseEnd
                            if syms.peek() != Some(&&Symbol::PhraseEnd) {
                                panic!("Expected `;` following keyword `return`.");
                            }
                            // throw away PhraseEnd
                            syms.next();
                            self.main_tasks.push(Task::ExitBlock);
                        }
                        _ => todo!()
                    }
                }
                sym => { todo!("TODO ICE sym ({:?})", sym) }
            }
        }
        return Err(anyhow::Error::msg(
            "Expected a } to close the main block before the end of the file. (TODO: ANNOTATIONS)"
        ));
    }
}

#[derive(Debug)]
pub enum Definition {
    System { label: String },
    File { label: String },
    Library { name: String, version: String },
    Alias { label: String, points_to: Vec<String> },
    GlobalConstant { label: String, value: Value },
    Function { label: String, tasks: Vec<Task> },
}

#[derive(Debug)]
pub enum Task {
    Set { label: String, type_: Option<String>, value: Value },
    Call { label: String, arguments: Vec<Value> },
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
    fn from_symbols<'a, I>(syms: &mut Peekable<I>, end: Symbol) -> Self
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

#[derive(Debug, Clone)]
pub enum Value {
    Integer(Bigint),
    Float(Bigfloat),
    Complex(Bigcplx),
    String(String),
    Bool(bool),
    Label(String),
}
