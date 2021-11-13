use std::env;
use dawn::Dawn;

mod dawn;

fn main() {
    let mut dawn = Dawn::new("database.dawn".to_string());

    let mut args = env::args();

    let method = args.nth(1).expect("You must enter the method (get/set)");

    if method == "get" {
        let key = args.nth(0).expect("You must specify a key to get a value.");

        if let Some(v) = dawn.get(key) {
            println!("{}", v)
        }
    } else if method == "set" {
        for arg in args {
            if let Some(i) = arg.find(':') {
                dawn.set(arg[..i].to_string(), arg[i + 1..].to_string());
            }
        }

        dawn.flush().unwrap();
    } else if method == "del" {
        let key = args.nth(0).expect("You must specify a key to delete a value.");

        dawn.delete(key);
        dawn.flush().unwrap();
    }
}
