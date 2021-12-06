use crate::lifepaths::*;
use serde::Deserialize;
use serde_json::Value;

use std::alloc::LayoutError;
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub enum LPPError {
    KeyNotFound(String),
    WrongType(String, String),
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
        }
    }
}

impl Error for LPPError {}

pub fn read_lifepaths(file_path: &Path) -> Result<Vec<Lifepath>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let val: Value = serde_json::from_reader(reader)?;
    let mut lifepaths = Vec::new();
    if let Value::Object(map) = val {
        for (k, v) in map {
            if k.as_str() != "items" {
                continue;
            }
            if let Value::Array(a) = v {
                for l in a {
                    lifepaths.push(read_lifepath(&l)?);
                }
            }
        }
    } else {
        panic!("Unexpected input");
    }

    Ok(lifepaths)
}

pub fn read_lifepath(val: &Value) -> Result<Lifepath, Box<dyn Error>> {
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
    lifepath.ok_or(Box::from("oof".to_string()))
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

fn get_stat_boost(map: &serde_json::Map<String, Value>) -> Result<StatBoost, Box<dyn Error>> {
    let subtract = map
        .get("subtractStats")
        .ok_or("Unexpected Input")?
        .as_bool()
        .ok_or("Unexpected Input")?;
    let stat = map
        .get("statBoost")
        .ok_or("Unexpected Input")?
        .as_str()
        .ok_or("Unexpected Input")?;
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
