use firetrail::lifepaths::lp_parser::{parse_settings, read_setting};
use firetrail::lifepaths::LifepathLookup;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    if args.is_empty() {
        panic!("No file path provided.");
    }

    let mut lifepath_lookup = LifepathLookup::default();

    let file = File::open(&args[0]).unwrap();
    let reader = BufReader::new(file);

    let settings = parse_settings(reader, &mut lifepath_lookup);

    if let Err(e) = settings {
        println!("{}", e);
        return;
    }

    let settings = settings.unwrap();

    for s in settings {
        println!("{}\n", s);
    }
}
