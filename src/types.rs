const MAX_AMOUNT_OF_JOKES: i8 = 10;
const MAX_ID_RANGE: i16 = 1368;
pub const url: &str = "https://v2.jokeapi.dev/joke/";
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

pub enum ResponseFormat {
    Json,
    Xml,
    Yaml,
    Text,
}

impl Default for ResponseFormat {
    fn default() -> Self {
        ResponseFormat::Json
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
    pub fn validate(&self) {
        if self.0 > self.1 || self.1 > 1368 {
            panic!(
                "Unexpected id range value, max value allowed is {:}",
                MAX_ID_RANGE
            );
        }
    }
}
