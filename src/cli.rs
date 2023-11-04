use std::env::args;

use crate::parser::beach::Definition;

pub fn main() {
    println!("🏖️ beach v{}", env!("CARGO_PKG_VERSION"));
    let mut args = args();
    // We don't need the running executable's path (if it exists)
    args.next().unwrap();
    let first_arg = args.next();
    if let Some(first_arg) = first_arg {
        match first_arg.as_str() {
            "internal_test" => {
                use std::str::FromStr;
                let target_dir = std::path::PathBuf::from_str("/Users/thisjaiden/Desktop/beach/tests/hello_world.beach").unwrap();
                let data = std::fs::read_to_string(target_dir).unwrap();
                println!("Parsing...");
                let parsed_data = crate::parser::beach::parse_string_file(data);
                println!("{:#?}", parsed_data);
                println!("Abstracting...");
                let abstract_data = crate::parser::beach::abstract_syntax(parsed_data, None).unwrap();
                println!("{:#?}", abstract_data);
                println!("Generating IR...");
                let ir_data = crate::parser::beach::intermediate_representation(abstract_data);
                println!("{:#?}", ir_data);
                println!("Generating assembly...");
                let assembly_data = crate::platform::get_all_platforms()[0].generate_assembly(ir_data);
                println!("{}", assembly_data);
                return;
            }
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
    match input_file.try_exists() {
        Ok(true) => {
            if let Ok(data) = std::fs::read_to_string(input_file.clone()) {
                // TODO: build
                println!("👓 Parsing file...");
                let parsed_data = crate::parser::beach::parse_string_file(data);
                println!("🔎 Checking syntax...");
                let abstract_data = crate::parser::beach::abstract_syntax(parsed_data, None);
                if let Err(e) = abstract_data {
                    panic!("{}", e);
                }
                let mut program_data = abstract_data.unwrap();
                let mut potential_subfiles = true;
                let mut main_file = true;
                //let mut parsed_subfiles = vec![];
                while potential_subfiles {
                    potential_subfiles = false;
                    println!("👓➕ Parsing subfiles...");
                    for definition in program_data.definitions {
                        match definition {
                            Definition::File { mut label } => {
                                let mut active_path = input_file.clone();
                                let glob = label.ends_with("~~");
                                if glob {
                                    label = label.trim_end_matches("~~").to_string();
                                }
                                if main_file {
                                    active_path.pop();
                                    active_path.push(format!("{label}.beach"));
                                }
                                else {
                                    active_path.pop();
                                    active_path.push(format!(""))
                                }
                                if let Ok(data) = std::fs::read_to_string(active_path) {
                                    let parsed_data = crate::parser::beach::parse_string_file(data);
                                }
                                else {
                                    println!("Could not find file `{label}.beach` imported from the main file.");
                                    std::process::exit(0);
                                }
                            }
                            _ => {} // ignore
                        }
                    }
                    println!("🔎➕ Checking subfiles...");
                    todo!();
                    main_file = false;
                }
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
        Ok(false) => {
            println!("`main.beach` not found. Check your directory and try again.");
        }
        Err(_) => {
            println!("Unable to find or access `main.beach`. Check directory permissions and try again.");
        }
    }
}

fn info(args: &mut std::env::Args) {
    // TODO: auto generate this date on build
    println!("🕰️ Approximate build date: November 2023");
    // TODO: error handling
    println!("🔍Executable located at {}", std::env::current_exe().unwrap().display());
    // TODO: change when repo is made public
    println!("🛠️ Closed Alpha. Do not redistribute.");
    // I understand *this* repository was not created in 2019, but some code in
    // this repository is migrated from other locations datestamped as far back
    // as 2019, thus grandfathering in the date.
    println!("©️ Created and (c) Jaiden Bernard 2019-2023.");
}
