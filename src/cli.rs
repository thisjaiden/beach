use std::env::args;

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
                let abstract_data = crate::parser::beach::abstract_syntax(parsed_data);
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
debug");
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
                todo!();
            }
            inv => {
                println!("Invalid argument. (`{inv}`) Try `beach help build` for a list of valid arguments.");
                std::process::exit(0);
            }
        }
    }
    // TOCTOU ok here: We handle all error conditions gracefully. We're only
    // really checking to *improve* error messages, not *provide* them.
    // TODO: BACKLOG: Changing this to a match statement would decrease nesting,
    // improve code readability, and potentially slightly improve performance.
    if let Ok(exists) = input_file.try_exists() {
        if exists {
            if let Ok(data) = std::fs::read_to_string(input_file) {
                // TODO: build
                println!("👓 Parsing file...");
                let parsed_data = crate::parser::beach::parse_string_file(data);
                println!("🔎 Checking syntax...");
                todo!();
                loop {
                    println!("👓➕ Parsing subfiles...");
                    todo!();
                    println!("🔎➕ Checking subfiles...");
                    todo!();
                }
                println!("📖 Generating intermediates...");
                todo!();
                println!("🎛️ Calculating valid targets...");
                todo!();
                /*
                for target in valid_targets {
                    println!("🔨 Compiling for {}", target.name);
                    todo!();
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
        else {
            println!("`main.beach` not found. Check your directory and try again.");
        }
    }
    else {
        println!("Unable to find or access `main.beach`. Check directory permissions and try again.");
    }
}
