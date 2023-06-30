const MAX_AMOUNT_OF_JOKES: i8 = 10;
const MAX_ID_RANGE: i16 = 1368;
pub enum Category {
    Chirstmas,
    Dark,
    Misc,
    Programming,
    Pun,
    Spooky,
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

pub struct AmountOfJokes(i8);

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

pub struct IdRange(i16, i16);

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
