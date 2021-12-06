use std::env;
use firetrail::lifepaths::lp_parser::read_lifepaths;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    if args.is_empty() {
        panic!("No file path provided.");
    }

    let r = read_lifepaths(Path::new(&args[0]));
}
