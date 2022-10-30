use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use lib::lib::*;
use kuchiki::traits::TendrilSink;

struct AppState {
    database: Mutex<HashMap<String, HashSet<IndexedResource>>>,
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

//TODO: sufficiently simmilar word in search (algorithm)
#[post("/resource")]
async fn add_resource(
    data: web::Data<AppState>,
    resource: web::Json<CrawledResource>,
) -> impl Responder {
    //parse content
    let document = scraper::Html::parse_document(resource.content.as_str());
    let kuchiki_parser = kuchiki::parse_html().one(resource.content.as_str());

    //remove script, style and noscript tags
    kuchiki_parser
        .inclusive_descendants()
        .filter(|node| {
            node.as_element().map_or(false, |e| {
                matches!(e.name.local.as_ref(), "script" | "style" | "noscript")
            })
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|node| node.detach());

    let text = kuchiki_parser.text_contents();

    let split_words = text.split(' ');

    //fixup words (remove words with non alphabetic chars, empty words, transform to lowercase...)
    let fixed_words: Vec<String> = split_words
        .map(|w| w.to_ascii_lowercase().split_whitespace().collect())
        .filter(|w: &String| !w.is_empty())
        .collect();

    println!("xd: {:?}", fixed_words);

    let title_selector = scraper::Selector::parse("title").unwrap();
    let description_selector = scraper::Selector::parse("meta").unwrap();

    let page_title: String = document
        .select(&title_selector)
        .map(|e| e.inner_html())
        .take(1)
        .collect();

    let page_description: String = document
        .select(&description_selector)
        .filter(|e| e.value().attr("name") == Some("description"))
        .filter_map(|e| e.value().attr("content"))
        .take(1)
        .collect();

    //and for each changed content word we add it to the db (word -> list.append(url))
    let mut database = data.database.lock().unwrap();
    for word in &fixed_words {
        let resource_to_add = IndexedResource {
            url: resource.url.clone(),
            priority: calculate_word_priority(word, resource.content.as_str(), &fixed_words),
            word: Arc::new(word.clone()),
            title: page_title.clone(),
            description: page_description.clone(),
        };

        match database.get_mut(word) {
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

    //percentage of valid words
    let mut valid_results: Option<HashSet<IndexedResource>> = None;
    for w in query {
        //Normalise queries to lowercase
        let w = w.to_ascii_lowercase();

        let curr_word_results = match search_word_in_db(&database, w.to_string()) {
            None => return "[]".to_string(),
            Some(curr_results) => curr_results,
        };

        match valid_results {
            //Initialise valid_results
            None => {
                valid_results = Some(curr_word_results.to_owned());
            }
            Some(results) => {
                let intersection: HashSet<IndexedResource> = curr_word_results
                    .intersection(&results)
                    .map(|s| s.to_owned())
                    .collect();
                valid_results = Some(intersection);
            }
        }
    }

    serde_json::to_string(&valid_results.unwrap()).unwrap()
}

fn search_word_in_db(
    db: &HashMap<String, HashSet<IndexedResource>>,
    word: String,
) -> Option<&HashSet<IndexedResource>> {
    db.get(&word)
}

fn calculate_word_priority(word: &str, _html_site: &str, words: &[String]) -> u32 {
    //TODO: priorize lower levels of url, priorize word in url/title/description or main?

    //atm priority is just the number of occurences in the site.
    words.iter().filter(|w| *w == word).count() as u32
}
