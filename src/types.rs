use serde::Deserialize;
use std::collections::HashMap;

use crate::constants::*;
pub enum Category {
    Chirstmas,
    Dark,
    Misc,
    Programming,
    Pun,
    Spooky,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::Chirstmas => String::from("Chirstmas"),
            Category::Dark => String::from("Dark"),
            Category::Misc => String::from("Misc"),
            Category::Programming => String::from("Programming"),
            Category::Pun => String::from("Pun"),
            Category::Spooky => String::from("Spooky"),
        }
    }
}

pub enum Language {
    Czech,
    English,
    French,
    German,
    Portuguese,
    Spanish,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

pub enum Flags {
    Nsfw,
    Religious,
    Political,
    Racist,
    Sexist,
    Explicit,
}

impl ToString for Flags {
    fn to_string(&self) -> String {
        match self {
            Flags::Nsfw => String::from("Nsfw"),
            Flags::Religious => String::from("Religious"),
            Flags::Political => String::from("Political"),
            Flags::Racist => String::from("Racist"),
            Flags::Sexist => String::from("Sexist"),
            Flags::Explicit => String::from("Explicit"),
        }
    }
}

pub enum Format {
    Json,
    Xml,
    Yaml,
    Text,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

pub enum JokeType {
    Any,
    Single,
    TwoPart,
}

impl Default for JokeType {
    fn default() -> Self {
        JokeType::Any
    }
}

pub struct AmountOfJokes(pub i8);

impl AmountOfJokes {
    pub fn validate(&self) {
        if self.0 > 10 {
            panic!(
                "Max amount of jokes can be {} got {}",
                MAX_AMOUNT_OF_JOKES, self.0
            );
        }
    }
}

impl Default for AmountOfJokes {
    fn default() -> Self {
        Self(1)
    }
}

pub struct IdRange(pub i16, pub i16);

impl IdRange {
    pub fn validate(&self, language: &Language) {
        if self.0 > self.1 || self.1 > 1368 {
            panic!(
                "Unexpected id range value, max value allowed is {:}",
                MAX_ID_RANGE
            );
        }
        Self::language_range(self.1, &language);
    }

    pub fn language_range(id: i16, language: &Language) {
        match language {
            Language::Czech => assert!(id <= language_range::CZECH),
            Language::French => assert!(id <= language_range::FRENCH),
            Language::German => assert!(id <= language_range::GERMAN),
            Language::Portuguese => assert!(id <= language_range::PORTUGUESE),
            Language::Spanish => assert!(id <= language_range::SPANISH),
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    Json(String),
    Standard(StandardResponseType),
}

#[derive(Debug, Deserialize)]
pub struct StandardResponseType {
    #[serde(flatten)]
    #[serde(rename = "type")]
    pub joketype: JokePart,
    pub error: bool,
    pub category: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum JokePart {
    #[serde(rename = "twopart")]
    Twopart(MultiplePartJoke),
    #[serde(rename = "single")]
    Single(SinglePartJoke),
}

#[derive(Debug, Deserialize)]
pub struct MultiplePartJoke {
    pub setup: String,
    pub delivery: String,
    pub flags: StandardFlag,
    pub id: i16,
    pub safe: bool,
    pub lang: String,
}

#[derive(Debug, Deserialize)]
pub struct SinglePartJoke {
    pub joke: Option<String>,
    pub flags: HashMap<String, bool>,
    pub id: i16,
    pub safe: bool,
    pub lang: String,
}

#[derive(Debug, Deserialize)]
pub struct StandardFlag {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}
