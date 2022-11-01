mod indexer_implementation;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use indexer_implementation::IndexerImplementation;
use kuchiki::traits::TendrilSink;
use lib::lib::*;
use std::collections::HashSet;
use std::sync::Mutex;

pub trait Indexer {
    //too many args?
    fn insert(
        &mut self,
        word: &str,
        url: &str,
        title: &str,
        description: &str,
        content: &str,
        fixed_words: &[String],
    ) -> Result<(), String>;
    fn search(&self, term: &str) -> Result<HashSet<IndexedResource>, String>;
    fn num_of_words(&self) -> usize;
}

struct AppState {
    indexer: Mutex<Box<dyn Indexer + Send + Sync>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world! Im the indexer!");

    serve_http_endpoint("0.0.0.0", 4444).await
}

async fn serve_http_endpoint(address: &str, port: u16) -> std::io::Result<()> {
    let shared_state = web::Data::new(AppState {
        indexer: Mutex::new(Box::new(IndexerImplementation::new())),
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
    let mut indexer = data.indexer.lock().unwrap();
    for word in &fixed_words {
        let _ = indexer.insert(
            word,
            &resource.url,
            &page_title,
            &page_description,
            &resource.content,
            &fixed_words,
        );
    }

    //TODO: ADD LANG? EN in meta tag (frontend)

    println!("Added resource: {:?}", indexer.num_of_words());

    format!("{resource:?}")
}

#[get("/search")]
async fn no_search(_data: web::Data<AppState>) -> impl Responder {
    "[]".to_string()
}

#[get("/search/{term}")]
async fn search(data: web::Data<AppState>, term: web::Path<String>) -> impl Responder {
    let indexer = data.indexer.lock().unwrap();

    let results = indexer.search(&term);

    serde_json::to_string(&results.unwrap()).unwrap()
}
