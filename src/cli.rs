use std::env::args;

pub fn main() {
    let mut args = args();
    // We don't need the running executable's path (if it exists)
    args.next().unwrap();
    let first_arg = args.next();
    if let Some(first_arg) = first_arg {
        todo!();
    }
    else {
        println!("No arguments passed. Try `beach help` for more info.");
    }
}
