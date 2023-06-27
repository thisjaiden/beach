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
                let parsed_data = crate::parser::beach::parse_string_file(data);
                println!("{:#?}", parsed_data);
                println!("ABSTRACTING...");
                let abstract_data = crate::parser::beach::abstract_syntax(parsed_data);
                println!("{:#?}", abstract_data);
                return;
            }
            "build" => {
                build(&mut args);
            }
            _ => todo!()
        }
    }
    else {
        println!("No arguments passed. Try `beach help` for more info.");
    }
}

fn build(args: &mut std::env::Args) {
    // TODO: no unwrap!
    let calling_dir = std::env::current_dir().unwrap();
    // Seach for a `main.beach` file.
    let mut main_file = calling_dir.clone();
    main_file.push("main.beach");
    // TOCTOU ok here
    if let Ok(exists) = main_file.try_exists() {
        if exists {
            if let Ok(data) = std::fs::read_to_string(main_file) {
                // TODO: build
                println!("👓 Parsing file...");
                let parsed_data = crate::parser::beach::parse_string_file(data);
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
