use std::collections::HashMap;
use crate::lifepaths::lp_parser::LifepathBuilder;

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

pub enum StatBoost {
    None,
    Mental(i8),
    Physical(i8),
    Both(i8),
    Either(i8)
}

struct Requirement {
    str: String
}

struct Restriction {
    str: String
}

struct Note {
    str: String
}

pub struct Lifepath {
    time: u16,
    resources: u16,
    stat_boost: StatBoost,
    leads: Vec<String>,
    skill_points: u16,
    general_points: u16,
    trait_points: u16,
    skill_list: Vec<String>,
    trait_list: Vec<String>,
    requirements: String,
    restrictions: String,
    note: String,
}