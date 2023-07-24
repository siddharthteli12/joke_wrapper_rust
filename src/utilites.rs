use crate::types::*;
use std::vec;

pub fn enum_join<T: ToString>(list_enum: &Vec<T>) -> Vec<String> {
    let mut list_string = vec![];
    for item in list_enum {
        list_string.push(item.to_string());
    }
    list_string
}

pub fn build_category(list: &Vec<Category>, joke_url: &mut String) {
    match list.len() {
        0 => {
            joke_url.push_str("Any?");
        }
        _ => {
            joke_url.push_str(&enum_join(&list).join(","));
            joke_url.push('?');
        }
    }
}

pub fn build_language(language: &Language, joke_url: &mut String) {
    match language {
        Language::Czech => joke_url.push_str("lang=cs&"),
        Language::English => {}
        Language::French => joke_url.push_str("lang=fr&"),
        Language::German => joke_url.push_str("lang=de&"),
        Language::Portuguese => joke_url.push_str("lang=pt&"),
        Language::Spanish => joke_url.push_str("lang=es&"),
    }
}

pub fn build_flags(list: &Vec<Flags>, joke_url: &mut String) {
    if list.len() > 0 {
        let value = format!("blacklistFlags={:}&", enum_join(list).join(","));
        joke_url.push_str(&value);
    }
}

pub fn build_response_format(response: &Format, joke_url: &mut String) {
    match response {
        Format::Json => {}
        Format::Xml => joke_url.push_str("format=xml&"),
        Format::Yaml => joke_url.push_str("format=yaml&"),
        Format::Text => joke_url.push_str("format=txt&"),
    }
}

pub fn build_joke_type(joke_type: &JokeType, joke_url: &mut String) {
    match joke_type {
        JokeType::Any => {}
        JokeType::Single => joke_url.push_str("type=single&"),
        JokeType::TwoPart => joke_url.push_str("type=twopart&"),
    }
}

pub fn build_search_string(search_string: &Option<&str>, joke_url: &mut String) {
    if let Some(string) = &search_string {
        if !string.is_empty() {
            let value = format!("contains={}&", string);
            joke_url.push_str(&value);
        }
    }
}

pub fn build_range(range: &Option<IdRange>, joke_url: &mut String) {
    if let Some(range) = &range {
        let value = format!("idRange={:}-{:}&", range.0, range.1);
        joke_url.push_str(&value);
    }
}

pub fn build_amount_of_jokes(amount_of_jokes: i8, joke_url: &mut String) {
    if amount_of_jokes > 1 {
        let value = format!("amount={:}", amount_of_jokes);
        joke_url.push_str(&value);
    }
}
