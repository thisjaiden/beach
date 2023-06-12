use std::env::args;

pub fn main() {
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
                return;
            }
            _ => {}
        }
        todo!();
    }
    else {
        println!("No arguments passed. Try `beach help` for more info.");
    }
}
