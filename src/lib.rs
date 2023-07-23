#![allow(dead_code)]

pub mod types;
use core::panic;

pub use types::*;

mod utilites;
use utilites::*;

use ureq;
pub struct Joke {
    category: Vec<Category>,
    language: Language,
    flags: Vec<Flags>,
    response_format: Format,
    joke_type: JokeType,
    search_string: Option<String>,
    id_range: Option<IdRange>,
    amount_of_jokes: AmountOfJokes,
}

impl Joke {
    pub fn new() -> Self {
        Self {
            category: vec![],
            language: Language::English,
            flags: vec![],
            response_format: Format::default(),
            joke_type: JokeType::default(),
            search_string: None,
            id_range: None,
            amount_of_jokes: AmountOfJokes::default(),
        }
    }

    pub fn language(&mut self, language: Language) -> &mut Self {
        self.language = language;
        self
    }

    pub fn flags(&mut self, flags: Vec<Flags>) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn response_format(&mut self, response_format: Format) -> &mut Self {
        self.response_format = response_format;
        self
    }

    pub fn joke_type(&mut self, joke_type: JokeType) -> &mut Self {
        self.joke_type = joke_type;
        self
    }

    pub fn search_string(&mut self, search_string: String) -> &mut Self {
        self.search_string = Some(search_string);
        self
    }

    pub fn id_range(&mut self, id_range: IdRange) -> &mut Self {
        id_range.validate(&self.language);
        self.id_range = Some(id_range);
        self
    }

    pub fn amount_of_jokes(&mut self, amount_of_jokes: AmountOfJokes) -> &mut Self {
        amount_of_jokes.validate();
        self.amount_of_jokes = amount_of_jokes;
        self
    }

    pub fn build(&mut self) -> JokeUrl {
        let mut joke_url = JokeUrl::get_url().0;

        build_category(&self.category, &mut joke_url);

        build_language(&self.language, &mut joke_url);

        build_flags(&self.flags, &mut joke_url);

        build_response_format(&self.response_format, &mut joke_url);

        build_joke_type(&self.joke_type, &mut joke_url);

        build_search_string(&self.search_string, &mut joke_url);

        build_range(&self.id_range, &mut joke_url);

        build_amount_of_jokes(self.amount_of_jokes.0, &mut joke_url);

        if joke_url.ends_with('&') || joke_url.ends_with('?') {
            joke_url.pop();
        }
        JokeUrl(joke_url)
    }
}

pub struct JokeUrl(String);

impl JokeUrl {
    pub fn get_url() -> Self {
        JokeUrl("https://v2.jokeapi.dev/joke/".to_string())
    }

    pub fn get_joke(self, standard: bool) -> Result<ResponseType, ureq::Error> {
        let body: String = ureq::get(&self.0).call()?.into_string()?;
        if standard {
            if self.0.contains("format") {
                panic!("Format can only set to json when standard flag is true");
            }
            let response: StandardResponseType = serde_json::from_str(&body).unwrap();
            Ok(ResponseType::Standard(response))
        } else {
            Ok(ResponseType::Json(body))
        }
    }
}
