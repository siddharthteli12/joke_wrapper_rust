use joke_api_rust::{types::*, Flags::*, Joke};

fn main() {
    let joke = Joke::new()
        .flags(vec![Nsfw, Religious])
        .build()
        .get_joke(true)
        .unwrap();
    match joke {
        ResponseType::Standard(joke) => {
            println!(" \n {:?} \n", joke);
        }
        _ => {}
    }
}
