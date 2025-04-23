use std::env::args;

use crate::{parser::beach::ast::user_token_format::{keywords::Keyword, Symbol}, utils::install_directory};

pub fn main() {
    println!("beach 🏖️  v{}", env!("CARGO_PKG_VERSION"));

    let mut args = args();
    // We don't need the running executable's path (if it exists)
    args.next().unwrap();

    // The first argument passed to beach should be the main task of this
    // execution cycle.
    let first_arg = args.next();
    if let Some(first_arg) = first_arg {
        match first_arg.as_str() {
            "build" => build(&mut args),
            "help" => help(&mut args),
            "info" => info(&mut args),
            _ => todo!()
        }
    }
    else {
        println!("No arguments passed. Try `beach help` for more info.");
    }
}

fn help(args: &mut std::env::Args) {
    // TODO: read args for more specific help
    match args.next().as_deref() {
        Some("build") => {
            println!("\
beach build [ARGS]
Avalable arguments:
input [FILE]
output folder [FOLDER]
output name [NAME]
debug
assembly");
        }
        Some(_) => {
            println!("Unknown argument. Try `beach help` for a list of options.");
            todo!();
        }
        None => {
            println!("\
General usage:
beach build - builds your program for many platforms
beach run - builds and runs your program on this platform
beach check - checks if your program is valid
beach update - updates the compiler, if possible
beach info - gives information about the current installation
beach help - provides this help menu
beach help [COMMAND] - provides more detailed help");
        }
    }
}

fn build(args: &mut std::env::Args) {
    // TODO: no unwrap!
    let mut input_file = std::env::current_dir().unwrap();
    input_file.push("main.beach");
    // Indicates if the build process should output the generated assembly for
    // each given platform.
    let mut output_assembly = false;
    // Indicates if debug symbols should be included in the final executables.
    let mut enable_debug_symbols = false;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "input" => {
                // TODO: no unwrap!
                input_file = std::env::current_dir().unwrap();
                input_file.push(args.next().expect("Expected a filepath following `input`."));
            }
            "output" => {
                todo!();
            }
            "debug" => {
                enable_debug_symbols = true;
                todo!("not used");
            }
            "assembly" => {
                output_assembly = true;
                todo!("not used");
            }
            inv => {
                println!("Invalid argument. (`{inv}`) Try `beach help build` for a list of valid arguments.");
                std::process::exit(0);
            }
        }
    }
    // TOCTOU ok here: We handle all error conditions gracefully. We're only
    // really checking to *improve* error messages, not *provide* them.
    let file_exists = input_file.try_exists();
    if let Err(_e) = file_exists {
        // TODO: check input filename for error
        println!("Unable to find or access `main.beach`. Check directory permissions and try again.");
        std::process::exit(0);
    }
    if let Ok(false) = file_exists {
        // TODO: check input filename for error
        println!("`main.beach` not found. Check your directory and try again.");
        std::process::exit(0);
    }

    if let Ok(data) = std::fs::read_to_string(input_file.clone()) {
        // Import std:core into file directly
        let mut dir = install_directory();
        dir.push("stdlib");
        dir.push("core.beach");
        let std_core = std::fs::read_to_string(dir).unwrap();
        // Reassign file with std imported
        let data = std_core + &data;

        // Parse file to ast
        println!("👓 Parsing main file...");
        let parsed_data = crate::parser::parse_string_file(data);

        /*
         * Find and parse subfiles
         * =======================
         * This section steps through every current unprocessed file to find all
         * imports and refrences. If an import hasn't yet been calculated
         * elsewhere, it's added as another file to process. This repeats until
         * there are no remaining unprocessed files.
         * 
         * This process does not verify correct syntax or logic and may
         * erroniously import files in the case of incomplete or incorrect input
         * code. Treat its output gently and start with the main project file.
         */
        
        // Main file's name (main.beach, alternate_main_file_name.beach, etc.)
        let input_file_ending = input_file.file_name().unwrap().to_str().unwrap().to_string();
        // A list of all files that have not yet been processed for looking for
        // imports. They are stored in the relative path format (pathname
        // relative to project root, host platform style dividers)
        let mut potential_subfiles_in = vec![input_file_ending.clone()];
        // Files that have been aquired from disk. This vector contains both
        // parsed and unparsed files.
        // Vec<(relative path format, ast)>
        let mut current_files = vec![(input_file_ending.clone(), parsed_data)];
        // A list of all files that have been aquired into `current_files`. This
        // vector contains both parsed and unparsed files as Strings with the
        // relative path format.
        let mut file_names = vec![input_file_ending.clone()];

        // As long as we have at least one file left to check, we should keep going!
        while !potential_subfiles_in.is_empty() {
            // Why use a manual index variable for the `'outer` for loop?
            // Because we remove elements from it mid-iteration! Not really the
            // best idea... but I'm not super sure how I want to do this yet so
            // this gets the job done.
            let mut idx = 0;
            'outer: for loc in &potential_subfiles_in.clone() {
                println!("👓 Parsing files ({}/{})...", potential_subfiles_in.len(), current_files.len());
                for (locof, syntax) in &current_files.clone() {
                    if locof == loc {
                        /* 
                         * Okay, we have two loops, `'outer` and what we'll call
                         * `'inner` for explanation's sake.
                         * 
                         * 'outer goes over every file we still need to look at.
                         * 'inner goes over every file we've aquired the ast for.
                         * 
                         * This *should* be more than zero files unless we've
                         * parsed every file in `potential_subfiles`.
                         * 
                         * If 'outer and 'inner are matching the same file, we
                         * handle the file we've found. Once we've handled that
                         * file, we break out of 'outer to avoid the funkiness
                         * of iterating over a clone of a list we just modified
                         * the original of.
                         * 
                         * This restarts us at the top of the while loop.
                         */
                        let mut symbols = syntax.symbols.iter();
                        while let Some(symbol) = symbols.next() {
                            if Symbol::Keyword(Keyword::Kfile) == *symbol {
                                // Local imports using `file`
                                let target = symbols.next();
                                if let Some(Symbol::Label(lbl)) = target {
                                    let glob = lbl.ends_with(":*");
                                    let mut working_pathized = lbl.clone();
                                    if glob {
                                        working_pathized = working_pathized.trim_end_matches(":*").to_string();
                                        todo!();
                                    }
                                    #[cfg(target_family = "unix")]
                                    { working_pathized = working_pathized.replace(":", "/") }
                                    #[cfg(not(target_family = "unix"))]
                                    { working_pathized = working_pathized.replace(":", "\\") }

                                    let mut file_path = input_file.clone();
                                    file_path.pop();
                                    file_path = file_path.join(locof);
                                    file_path.pop();
                                    file_path = file_path.join(working_pathized);
                                    let file_string = file_path.clone().into_os_string().to_string_lossy().to_string();
                                    current_files.push(
                                        (file_string.clone(),
                                        crate::parser::parse_string_file(std::fs::read_to_string(file_path.clone()).unwrap()))
                                    );
                                    if !file_names.contains(&file_string) {
                                        potential_subfiles_in.push(file_string.clone());
                                        file_names.push(file_string.clone());
                                    }
                                }
                                else {
                                    println!("Failed parsing.");
                                    std::process::exit(0);
                                }
                            }
                            else if Symbol::Keyword(Keyword::Kinclude) == *symbol {
                                // Standard library imports using `include`
                                let target = symbols.next();
                                let mut next_target = symbols.next();
                                let mut total_label = String::new();
                                if let Some(Symbol::Label(lbl)) = target {
                                    total_label += lbl;
                                }
                                else {
                                    // TODO: handle this case
                                    todo!();
                                }
                                // TODO: handle invalid symbols and premature end
                                while next_target != Some(&Symbol::PhraseEnd) && next_target.is_some() {
                                    if next_target == Some(&Symbol::Is) {
                                        #[cfg(target_family = "unix")]
                                        { total_label += "/"; }
                                        #[cfg(not(target_family = "unix"))]
                                        { total_label += "\\"; }
                                    }
                                    else if let Some(Symbol::Label(lbl)) = next_target {
                                        total_label += lbl;
                                    }
                                    next_target = symbols.next();
                                }

                                let mut file_path = install_directory();
                                file_path.push("stdlib");
                                file_path.push("std");
                                file_path = file_path.join(&total_label);
                                file_path.set_extension("beach");
                                if !file_path.try_exists().is_ok_and(|v| v) {
                                    file_path.pop();
                                    file_path.set_extension("beach");
                                }
                                // TODO: verify that the path one level up actually exists
                                
                                let file_string = file_path.clone().into_os_string().to_string_lossy().to_string();
                                //println!("in {}", locof);
                                //println!("current file tokens: {:#?}", current_files[0].1);
                                //println!("looking for all: {:?}", potential_subfiles_in);
                                //println!("have the following: {:?}", file_names);
                                //println!("label: {}", total_label);
                                //println!("Searching for: {:?}", file_path);
                                current_files.push(
                                    (file_string.clone(),
                                    crate::parser::parse_string_file(std::fs::read_to_string(file_path.clone()).unwrap()))
                                );
                                if !file_names.contains(&file_string) {
                                    potential_subfiles_in.push(file_string.clone());
                                    file_names.push(file_string.clone());
                                }
                            }
                            else if let Symbol::Compiler(val) = symbol {
                                // Internal compiler imports using `!!core`
                                let mut iter = val.split(" ");
                                if iter.next() == Some("core") {
                                    let path = iter.next().unwrap();
                                    #[cfg(not(target_family = "unix"))]
                                    let path = path.replace("/", "\\");
                                    let mut dirpath = install_directory();
                                    dirpath.push("stdlib");
                                    dirpath.push("core");
                                    dirpath.push(format!("{}.beach", path));
                                    let file_string = dirpath.to_str().unwrap().to_string();
                                    current_files.push(
                                        (file_string.clone(),
                                        crate::parser::parse_string_file(std::fs::read_to_string(dirpath.clone()).unwrap()))
                                    );
                                    if !file_names.contains(&file_string) {
                                        potential_subfiles_in.push(file_string.clone());
                                        file_names.push(file_string.clone());
                                    }
                                }
                            }
                        }
                        // cleanse the list!
                        potential_subfiles_in.remove(idx);
                        break 'outer;
                    }
                }
                idx += 1;
            }
            if idx.checked_sub(1) == Some(potential_subfiles_in.len()) {
                panic!("Couldn't find appropriate location!");
            }
            //println!("All filenames: {:?}", file_names);
            //println!("Looking for files: {:?}", potential_subfiles_in);
            //println!("Parsed files: {:?}", current_files);
        }
        println!("DEBUG / All filenames: {:#?}", file_names);
        println!("📖 Generating intermediates...");
        todo!();
        println!("🎛️ Calculating valid targets...");
        todo!();
        /*
        for target in valid_targets {
            println!("🔨 Compiling for {}", target.name);
            todo!();
            if output_assembly {
                todo!();
            }
            for packager in target.packagers {
                println!("📦 Packaging as {}...", packager.name);
                todo!();
            }
        }
        println!(
            "☑️ Built for {} targets in {}",
            valid_targets.len(),
            elapsed_build_time
        );
        */
    }
    else {
        println!("`main.beach` is not valid UTF-8 or otherwise could not be read.");
    }
}

fn info(_args: &mut std::env::Args) {
    // TODO: auto generate this date on build
    println!("🕰️ Approximate build date: April 2025");
    // TODO: error handling
    println!("🔍Executable located at {}", std::env::current_exe().unwrap().display());
    // I understand *this* repository was not created in 2019, but some code in
    // this repository is migrated from other locations datestamped as far back
    // as 2019, thus grandfathering in the date.
    println!("©️ Created and (c) Jaiden Bernard 2019-2025.");
}
