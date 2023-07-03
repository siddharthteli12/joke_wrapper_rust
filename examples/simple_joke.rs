use joke_api_rust::{types::*, Joke};

fn main() {
    let joke = Joke::new().build().get_joke(true).unwrap();
    match joke {
        ResponseType::Standard(joke) => {
            println!(" \n {:?} \n", joke);
        }
        _ => {}
    }
}
