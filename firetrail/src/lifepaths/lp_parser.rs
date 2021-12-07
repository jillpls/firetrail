use crate::lifepaths::*;
use serde::Deserialize;
use serde_json::{Number, Value};

use std::alloc::LayoutError;
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse_settings(
    reader: BufReader<File>,
    lifepath_lookup: &mut LifepathLookup,
) -> Result<Vec<Setting>, LPPError> {
    let mut settings = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        settings.push(read_setting(lifepath_lookup, &line)?);
    }
    Ok(settings)
}

pub fn read_setting(
    lifepath_lookup: &mut LifepathLookup,
    setting_str: &str,
) -> Result<Setting, LPPError> {
    let val: Value = serde_json::from_str(setting_str).or(Err(LPPError::Unknown))?;

    let setting = if let Value::Object(map) = val {
        let name = {
            map["name"]
                .as_str()
                .ok_or(LPPError::WrongType(
                    "String".to_string(),
                    "name".to_string(),
                ))?
                .clone()
        };
        println!("\n{}", name);
        let mut setting = Setting::new(name.to_string());
        for (k, v) in map {
            if k.as_str() != "items" {
                continue;
            }
            if let Value::Array(a) = v {
                let mut a = a.clone();
                a.sort_by(|l, m| {
                    let order_l = extract_order(l).unwrap_or(i64::MAX);
                    let order_m = extract_order(m).unwrap_or(i64::MAX);
                    order_l.cmp(&order_m)
                });
                for l in a {
                    println!(
                        "{}",
                        &l.as_object()
                            .unwrap()
                            .get("name")
                            .unwrap()
                            .as_str()
                            .unwrap()
                    );
                    let lifepath = read_lifepath(&l)?;
                    setting.lifepaths.push(lifepath.name.clone());
                    lifepath_lookup.add_lifepaths(lifepath, &setting.name);
                }
            }
        }
        println!("\n");
        setting
    } else {
        panic!("Unexpected input");
    };

    Ok(setting)
}

fn extract_order(val: &Value) -> Result<i64, LPPError> {
    val.as_object()
        .ok_or(LPPError::WrongType("object".to_string(), String::new()))?
        .get("data")
        .ok_or(LPPError::KeyNotFound("data".to_string()))?
        .as_object()
        .ok_or(LPPError::WrongType(
            "object".to_string(),
            "order".to_string(),
        ))?
        .get("order")
        .ok_or(LPPError::KeyNotFound("order".to_string()))?
        .as_i64()
        .ok_or(LPPError::WrongType("int".to_string(), "order".to_string()))
}

pub fn read_lifepath(val: &Value) -> Result<Lifepath, LPPError> {
    let lifepath: Option<Lifepath> = if let Value::Object(map) = val {
        let name = unwrap_string(map, "name")?;
        let data = unwrap_or_error(map, "data")?
            .as_object()
            .ok_or(LPPError::WrongType(
                "object".to_string(),
                "data".to_string(),
            ))?;
        let time = unwrap_int(data, "time")?;
        let resources = unwrap_int(data, "resources")?;
        let skill_points = unwrap_int(data, "skillPoints")?;
        let trait_points = unwrap_int(data, "traitPoints")?;
        let general_points = unwrap_int(data, "generalPoints")?;
        let stat_boost = get_stat_boost(data)?;
        let leads = split_list(data, "leads")?;
        let leads = if leads.is_empty() {
            Leads::None
        } else if leads[0].as_str() == "Any" {
            Leads::Any
        } else {
            Leads::Some(leads)
        };
        let skill_list = split_list(data, "skillList")?;
        let trait_list = split_list(data, "traitList")?;
        let restrictions = unwrap_string(data, "restrictions")?;
        let restrictions = if restrictions.is_empty() {
            None
        } else {
            Some(vec![Restriction::Custom(restrictions)])
        };
        let requirements = unwrap_string(data, "requirements")?;
        let requirements = if requirements.is_empty() {
            None
        } else {
            Some(vec![Requirement::Custom(requirements)])
        };
        let note = unwrap_string(data, "note")?;
        let note = if note.is_empty() {
            None
        } else {
            Some(vec![Note::Custom(note)])
        };
        Some(Lifepath::new(
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
        ))
    } else {
        None
    };
    lifepath.ok_or(LPPError::Unknown)
}

fn unwrap_int(map: &serde_json::Map<String, Value>, key: &str) -> Result<i64, LPPError> {
    let val = unwrap_or_error(map, key)?;
    if val.is_null() {
        return Ok(0);
    }
    val.as_i64()
        .ok_or(LPPError::WrongType("int".to_string(), key.to_string()))
}

fn unwrap_string(map: &serde_json::Map<String, Value>, key: &str) -> Result<String, LPPError> {
    let val = unwrap_or_error(map, key)?;
    Ok(val
        .as_str()
        .ok_or(LPPError::WrongType("String".to_string(), key.to_string()))?
        .to_string())
}

fn unwrap_or_error<'a>(
    map: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Result<&'a Value, LPPError> {
    map.get(key).ok_or(LPPError::KeyNotFound(key.to_string()))
}

fn split_list(map: &serde_json::Map<String, Value>, key: &str) -> Result<Vec<String>, LPPError> {
    let val = unwrap_or_error(map, key)?;
    let list = val
        .as_str()
        .ok_or(LPPError::WrongType("String".to_string(), key.to_string()))?
        .to_string();
    if list.is_empty() {
        return Ok(Vec::new());
    }
    let list: Vec<String> = list.split(",").map(|x| x.trim().to_string()).collect();
    Ok(list)
}

fn get_stat_boost(map: &serde_json::Map<String, Value>) -> Result<StatBoost, LPPError> {
    let subtract = map
        .get("subtractStats")
        .ok_or(LPPError::KeyNotFound("subtractStats".to_string()))?
        .as_bool()
        .ok_or(LPPError::WrongType(
            "bool".to_string(),
            "subtractStats".to_string(),
        ))?;
    let stat = map
        .get("statBoost")
        .ok_or(LPPError::KeyNotFound("statBoost".to_string()))?
        .as_str()
        .ok_or(LPPError::WrongType(
            "String".to_string(),
            "statBoost".to_string(),
        ))?;
    let bonus = if subtract { -1 } else { 1 };
    let stat = match stat {
        "physical" => StatBoost(bonus, StatBoostType::Physical),
        "mental" => StatBoost(bonus, StatBoostType::Mental),
        "both" => StatBoost(bonus, StatBoostType::Both),
        "either" => StatBoost(bonus, StatBoostType::Either),
        _ => StatBoost(0, StatBoostType::None),
    };

    Ok(stat)
}

#[derive(Debug)]
pub enum LPPError {
    KeyNotFound(String),
    WrongType(String, String),
    Unknown,
}

impl fmt::Display for LPPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyNotFound(s) => {
                write!(f, "Key not found: {}", s)
            }
            Self::WrongType(s, t) => {
                write!(f, "Wrong type, expected {} for {}", s, t)
            }
            _ => {
                write!(f, "Unknown Error")
            }
        }
    }
}

impl Error for LPPError {}
