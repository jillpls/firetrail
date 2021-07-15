use std::collections::HashMap;

pub mod lp_parser;

#[derive(Default)]
pub struct Setting {
    name: String,
    lifepaths: Vec<Lifepath>
}

#[derive(Default)]
pub struct LifepathLookup {
    lifepaths: HashMap<String, Lifepath>
}


pub struct Lifepath;
