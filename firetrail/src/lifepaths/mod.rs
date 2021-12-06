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

pub enum StatBoost {
    None,
    Mental(i8),
    Physical(i8),
    Both(i8),
    Either(i8)
}

pub enum Requirement {
    Custom(String)
}

pub enum Restriction {
    Custom(String)
}

pub enum Note {
    Custom(String)
}

pub enum Leads {
    Any,
    None,
    Some(Vec<String>)
}

pub struct Lifepath {
    name: String,
    time: u16,
    resources: u16,
    stat_boost: StatBoost,
    leads: Leads,
    skill_points: u16,
    general_points: u16,
    trait_points: u16,
    skill_list: Vec<String>,
    trait_list: Vec<String>,
    requirements: Option<Vec<Requirement>>,
    restrictions: Option<Vec<Restriction>>,
    note: Option<Vec<Note>>,
}