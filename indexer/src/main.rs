use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

struct AppState {
    database: Mutex<HashMap<String, HashSet<String>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world! Im the indexer!");

    serve_http_endpoint("0.0.0.0", 4444).await
}

async fn serve_http_endpoint(address: &str, port: u16) -> std::io::Result<()> {
    let shared_state = web::Data::new(AppState {
        database: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(greet)
            .service(add_resource)
    })
    .bind((address, port))?
    .run()
    .await
}

#[derive(Deserialize, Debug)]
struct Resource {
    url: String,
    content: String,
}

#[post("/resource")]
async fn add_resource(data: web::Data<AppState>, resource: web::Json<Resource>) -> impl Responder {
    //parse content
    let text = html2text::from_read(resource.content.as_str().as_bytes(), resource.content.len());

    let split_words = text.split(' ');

    //fixup words (remove words with non alphabetic chars, empty words, transform to lowercase...)
    let fixed_words: Vec<String> = split_words
        .filter(|w| !w.chars().any(|c| !c.is_ascii_alphabetic()))
        .filter(|w| !w.is_empty() && *w != " ")
        .map(|w| w.to_ascii_lowercase())
        .collect();

    println!("xd: {:?}", fixed_words);

    //and for each changed content word we add it to the db (word -> list.append(url))
    let mut database = data.database.lock().unwrap();
    for word in fixed_words {
        //should probs do some priority
        let maybe_urls = database.get_mut(&word);
        match maybe_urls {
            Some(urls) => _ = urls.insert(resource.url.clone()),
            None => _ = database.insert(word, HashSet::from([resource.url.clone()])),
        }
    }

    println!("Added resource! {:?}", database.len());
    format!("{:?}", resource)
}

#[get("/search/{term}")]
async fn greet(data: web::Data<AppState>, term: web::Path<String>) -> impl Responder {
    let query: Vec<&str> = term.split(' ').collect();
    let database = data.database.lock().unwrap();

    let mut valid_results: Option<HashSet<String>> = None;
    for w in query {
        let curr_word_results = database.get(w);
        if curr_word_results.is_none() {
            return format!("No results found for {:?}!", w);
        }
        let curr_word_results = curr_word_results.unwrap();
        match valid_results {
            None => {
                valid_results = Some(curr_word_results.clone());
            }
            Some(results) => {
                let intersection: Vec<String> = curr_word_results
                    .intersection(&results)
                    .map(|s| s.to_owned())
                    .collect();
                valid_results = Some(HashSet::from_iter(intersection));
            }
        }
    }

    format!(
        "Searching for: {term}\nResults: {:?}",
        valid_results.unwrap()
    )
}
