use std::collections::HashMap;

pub mod lp_parser;

#[derive(Default)]
pub struct Setting {
    name: String,
    lifepaths: Vec<Lifepath>,
}

#[derive(Default)]
pub struct LifepathLookup {
    lifepaths: HashMap<String, Lifepath>,
}

pub enum StatBoost {
    None,
    Mental(i8),
    Physical(i8),
    Both(i8),
    Either(i8),
}

pub enum Requirement {
    Custom(String),
}

pub enum Restriction {
    Custom(String),
}

pub enum Note {
    Custom(String),
}

pub enum Leads {
    Any,
    None,
    Some(Vec<String>),
}

pub struct Lifepath {
    name: String,
    time: i64,
    resources: i64,
    stat_boost: StatBoost,
    leads: Leads,
    skill_points: i64,
    general_points: i64,
    trait_points: i64,
    skill_list: Vec<String>,
    trait_list: Vec<String>,
    requirements: Option<Vec<Requirement>>,
    restrictions: Option<Vec<Restriction>>,
    note: Option<Vec<Note>>,
}

impl Lifepath {
    pub fn new(
        name: String,
        time: i64,
        resources: i64,
        stat_boost: StatBoost,
        leads: Leads,
        skill_points: i64,
        general_points: i64,
        trait_points: i64,
        skill_list: Vec<String>,
        trait_list: Vec<String>,
        requirements: Option<Vec<Requirement>>,
        restrictions: Option<Vec<Restriction>>,
        note: Option<Vec<Note>>
    ) -> Self {
        Self {
            name,
            time,
            resources,
            stat_boost,
            leads,
            skill_points,
            general_points,
            trait_points,
            skill_list,
            trait_list,
            requirements,
            restrictions,
            note
        }
    }
}
