use crate::lifepaths::*;

pub fn read_lifepaths(file_path: &str) -> LifepathLookup {
    let lookup = LifepathLookup::default();
    lookup
}

pub struct LifepathBuilder {
    time: u16,
    resources: u16,
    stat_boost: String,
    subtract_stats: bool,
    leads: String,
    skill_points: u16,
    general_points: u16,
    trait_points: u16,
    skill_list: String,
    trait_list: String,
    requirements: String,
    restrictions: String,
    note: String,
    order: i32
}

impl LifepathBuilder {
    pub fn build(self) -> Lifepath {
        Lifepath {
            time: self.time,
            resources: self.resources,
            stat_boost: generate_stat_boost(self.stat_boost, self.subtract_stats),
            leads: generate_leads(self.leads),
            skill_points: self.skill_points,
            general_points: self.general_points,
            trait_points: self.trait_points,
            skill_list: generate_skills(self.skill_list),
            trait_list: generate_traits(self.trait_list),
            requirements: self.requirements,
            restrictions: self.restrictions,
            note: self.note
        }
    }
}

fn generate_leads(str : String) -> Vec<String> {
    vec![]
}

fn generate_skills(str : String) -> Vec<String> {
    vec![]
}

fn generate_traits(str : String) -> Vec<String> {
    vec![]
}

fn generate_stat_boost(stat_boost : String, subtract : bool) -> StatBoost {
    let amount: i8 = if subtract { -1 } else { 1 };
    match stat_boost.as_str() {
        "mental" => StatBoost::Mental(amount),
        "physical" => StatBoost::Physical(amount),
        "both" => StatBoost::Both(amount),
        "either" => StatBoost::Either(amount),
        _ => StatBoost::None
    }

}