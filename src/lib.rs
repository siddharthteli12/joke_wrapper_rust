mod types;
use types::*;

struct Joke {
    category: Vec<Category>,
    language: Language,
    flags: Vec<Flags>,
    response_format: ResponseFormat,
    joke_type: JokeType,
    search_string: Option<String>,
    id_range: Option<i8>,
    amount_of_jokes: i8,
}
