use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

pub mod lp_parser;

#[derive(Default)]
pub struct Setting {
    name: String,
    lifepaths: Vec<String>,
}

impl Setting {
    pub fn new(name: String) -> Self {
        Self {
            name,
            lifepaths: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct LifepathLookup {
    lifepaths: HashMap<String, HashMap<String, Lifepath>>,
}

impl LifepathLookup {
    pub fn add_lifepaths(&mut self, lifepath: Lifepath, setting: &str) {
        if ! self.lifepaths.contains_key(&lifepath.name) {
            self.lifepaths.insert(lifepath.name.clone(), HashMap::new());
        }
        self.lifepaths.get_mut(&lifepath.name).unwrap().insert(setting.to_string(), lifepath);
    }

    pub fn get_lifepath(&self, name: &str, setting: &str) -> Option<&Lifepath> {
        if let Some(m) = self.lifepaths.get(name) {
            m.get(setting)
        } else {
            None
        }
    }

    pub fn lifepaths(&self) -> &HashMap<String, HashMap<String, Lifepath>> {
        &self.lifepaths
    }
}

pub struct StatBoost(i8, StatBoostType);

pub enum StatBoostType {
    Mental,
    Physical,
    Both,
    Either,
    None,
}

impl Display for StatBoost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self.1 {
            StatBoostType::Both => "M,P",
            StatBoostType::Either => "M/P",
            StatBoostType::Physical => "P",
            StatBoostType::Mental => "M",
            _ => return write!(f, "-"),
        };
        let sign = if self.0 < 0 { '-' } else { '+' };
        write!(f, "{}{}{}", sign, self.0, str)
    }
}

pub enum Requirement {
    Custom(String),
}

impl Display for Requirement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Requirement::Custom(s) = self {
            write!(f, "{}", s.as_str())
        } else {
            write!(f, "")
        }
    }
}

pub enum Restriction {
    Custom(String),
}

impl Display for Restriction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Restriction::Custom(s) = self {
            write!(f, "{}", s.as_str())
        } else {
            write!(f, "")
        }
    }
}

pub enum Note {
    Custom(String),
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Note::Custom(s) = self {
            write!(f, "{}", s.as_str())
        } else {
            write!(f, "")
        }
    }
}

pub enum Leads {
    Any,
    None,
    Some(Vec<String>),
}

impl Display for Leads {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(leads) => {
                write!(f, "{}", format_list(leads))
            }
            Self::Any => write!(f, "Any"),
            _ => write!(f, ""),
        }
    }
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
        note: Option<Vec<Note>>,
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
            note,
        }
    }
}

impl std::fmt::Display for Lifepath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let row1 = format!(
            "{}   {} yrs  {} res  {}  {}\n",
            self.name, self.time, self.resources, self.stat_boost, self.leads
        );
        let general = if self.general_points > 0 {
            format!(
                "{}{}: General; ",
                self.general_points,
                pt_pts(self.general_points)
            )
        } else {
            String::new()
        };
        let skills = format!(
            "{}{}: {}",
            self.skill_points,
            pt_pts(self.skill_points),
            format_list(&self.skill_list)
        );
        let traits = if self.trait_points > 0 {
            format!(
                "{}{}: {}",
                self.trait_points,
                pt_pts(self.trait_points),
                format_list(&self.trait_list)
            )
        } else {
            "-".to_string()
        };
        let row2 = format!("Skills: {}{}\n", general, skills);
        let row3 = format!("Traits: {}\n", traits);
        let requirements = if let Some(r) = &self.requirements {
            format!("Requirements: {}\n", format_list(r))
        } else {
            String::new()
        };
        let restrictions = if let Some(r) = &self.restrictions {
            format!("Restrictions: {}\n", format_list(r))
        } else {
            String::new()
        };
        let note = if let Some(r) = &self.note {
            format!("Notes: {}\n", format_list(r))
        } else {
            String::new()
        };
        let special = format!("{}{}{}", requirements, restrictions, note);
        write!(f, "{}{}{}{}", row1, row2, row3, special)
    }
}

fn format_list<T: Display>(list: &Vec<T>) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut list_str = String::from("");
    for l in list {
        list_str.push_str(format!("{}", l).as_str());
        list_str.push_str(", ");
    }

    let list_str = list_str.trim().trim_matches(',');
    format!("{}", list_str)
}

fn pt_pts(i: i64) -> String {
    if i > 1 {
        "pts".to_string()
    } else {
        "pt".to_string()
    }
}
