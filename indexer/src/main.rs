use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
struct CrawledResource {
    url: String,
    priority: u32, //how do we even calculate this
    word: Arc<String>,
}

//We implement PartialEq, Eq and Hash to ignore the priority field.
impl PartialEq for CrawledResource {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.word == other.word
    }
}
impl Eq for CrawledResource {}
impl Hash for CrawledResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.word.hash(state);
    }
}

struct AppState {
    database: Mutex<HashMap<String, HashSet<CrawledResource>>>,
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
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(shared_state.clone())
            .service(no_search)
            .service(search)
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
        let resource_to_add = CrawledResource {
            url: resource.url.clone(),
            priority: calculate_word_priority(&word, resource.content.as_str()),
            word: Arc::new(word.clone()),
        };

        match database.get_mut(&word) {
            Some(resources) => _ = resources.insert(resource_to_add),
            None => _ = database.insert(word.clone(), HashSet::from([resource_to_add])),
        }
    }

    println!("Added resource! {:?}", database.len());
    format!("{:?}", resource)
}

#[get("/search")]
async fn no_search(_data: web::Data<AppState>) -> impl Responder {
    "[]".to_string()
}

#[get("/search/{term}")]
async fn search(data: web::Data<AppState>, term: web::Path<String>) -> impl Responder {
    let query: Vec<&str> = term.split(' ').collect();
    let database = data.database.lock().unwrap();

    let mut valid_results: Option<HashSet<CrawledResource>> = None;
    for w in query {
        let curr_word_results = match database.get(w) {
            None => return "[]".to_string(),
            Some(results) => results,
        };

        match valid_results {
            None => {
                valid_results = Some(curr_word_results.to_owned());
            }
            Some(results) => {
                let intersection: HashSet<CrawledResource> = curr_word_results
                    .intersection(&results)
                    .map(|s| s.to_owned())
                    .collect();
                valid_results = Some(intersection);
            }
        }
    }

    serde_json::to_string(&valid_results.unwrap()).unwrap()
}

//TODO!
fn calculate_word_priority(_word: &str, _html_site: &str) -> u32 {
    rand::thread_rng().gen::<u32>()
}
