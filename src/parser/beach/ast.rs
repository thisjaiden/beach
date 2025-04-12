mod ast_types;
pub use ast_types::*;

mod global_scope;

pub mod user_token_format;

use std::{iter::Peekable, ops::Add};

use crate::utils::*;

use user_token_format::{Syntax, Symbol, keywords::Keyword};

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
    pub fn from_lst(lst: Syntax, prefix: Option<String>) -> Result<Program, anyhow::Error> {
        let mut program = Program {
            definitions: vec![],
            global_tasks: vec![],
            main_tasks: vec![],
            pending_file_additions: vec![]
        };
        let mut syms = lst.symbols.iter().peekable();
        global_scope::global_scope(&mut program, &mut syms)?;
        while !program.pending_file_additions.is_empty() {
            let mut glob_addition = String::new();
            for addition in &program.pending_file_additions {
                glob_addition += "\n";
                glob_addition += addition;
            }
            program.pending_file_additions.clear();
            let parsed = crate::parser::parse_string_file(glob_addition);
            let mut syms = parsed.symbols.iter().peekable();
            global_scope::global_scope(&mut program, &mut syms)?;
        }
        Ok(program)
    }
    fn type_declaration<'a, I>(&mut self, syms: &mut Peekable<I>) -> Result<Definition, anyhow::Error>
    where
        I: Iterator<Item = &'a Symbol> {
        if syms.peek().is_none() {
            return Err(anyhow::Error::msg(
                "Unexpected EOF following keyword `type`. (TODO: ANNOTATIONS)"
            ));
        }
        match syms.next().unwrap() {
            wrongsym => todo!("H: {:?}", wrongsym)
        }
    }
    fn trait_declaration<'a, I>(&mut self, syms: &mut Peekable<I>) -> Result<Definition, anyhow::Error>
    where
        I: Iterator<Item = &'a Symbol> {
        if syms.peek().is_none() {
            return Err(anyhow::Error::msg(
                "Unexpected EOF following keyword `trait`. (TODO: ANNOTATIONS)"
            ));
        }
        let trait_name;
        if let Symbol::Label(lbl) = syms.next().unwrap() {
            trait_name = lbl;
        }
        else {
            return Err(anyhow::Error::msg(
                "Expected a trait name following keyword `trait`. (TODO: ANNOTATIONS)"
            ));
        }
        if syms.next() != Some(&Symbol::OpenBrace) {
            return Err(anyhow::Error::msg(
                "Expected an opening brace following a trait declaration. (TODO: ANNOTATIONS)"
            ));
        }
        // TODO: loop over insides to get all the methods
        loop {
            // If there's no more symbols, we've hit an invalid EOF
            if syms.next().is_none() {
                return Err(anyhow::Error::msg(
                    "Unexpected EOF while parsing trait (TODO: ANNOTATIONS)"
                ));
            }
            // If we've hit a closing brace, we're done!
            if syms.next() == Some(&Symbol::CloseBrace) {
                break;
            }
        }
        todo!();
        return Ok(Definition::Trait {
            name: trait_name.to_string(),
            methods: vec![]
        })
    }
    fn compiler_directive<'a, I>(&mut self, syms: &mut Peekable<I>, data: &String) -> Result<(), anyhow::Error>
    where
        I: Iterator<Item = &'a Symbol> {
        if data == "core" {
            // we can ignore this directive, which is not used here.
        }
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
                                if let Definition::Alias {
                                    from: aptfr,
                                    to: aptto,
                                    export: _
                                } = &def {
                                    if l == aptfr {
                                        target = aptto.clone();
                                    }
                                }
                            }
                            self.main_tasks.push(Task::Call { function_identifier: target.clone(), arguments });
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
                                function_identifier: l.clone(),
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
