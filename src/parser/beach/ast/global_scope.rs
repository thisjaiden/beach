use std::iter::Peekable;

use crate::parser::beach::ast::user_token_format::Symbol;
use crate::parser::beach::ast::user_token_format::keywords::Keyword;

use super::Program;
use super::ast_types::*;

pub fn global_scope<'a, I>(
    program: &mut Program,
    syms: &mut Peekable<I>
) -> Result<(), anyhow::Error>
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
                                program.definitions.push(Definition::System { label: label.clone() });
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
                            program.main_scope(syms)?;
                        }
                        else {
                            return Err(anyhow::Error::msg(
                                "Expected `{` following keyword `main`. (TODO: ANNOTATIONS)"
                            ));
                        }
                    }
                    Keyword::Ktype => {
                        let def = program.type_declaration(syms)?;
                        program.definitions.push(def);
                    }
                    Keyword::Ktrait => {
                        let def = program.trait_declaration(syms)?;
                        program.definitions.push(def);
                    }
                    unknown_keyword => todo!("keyword {:?}", unknown_keyword)
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
                    program.global_tasks.push(Task::Call { function_identifier: l.to_string(), arguments });
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
                            program.definitions.push(Definition::Alias { from: l.clone(), to: out_lab_with_refs.join("~"), export: false });
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
            Symbol::Compiler(data) => {
                program.compiler_directive(syms, data)?;
            }
            k => todo!("{:?}", k)
        }
    }
    Ok(())
}
