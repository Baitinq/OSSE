mod indexer_implementation;

use actix_cors::Cors;
use actix_web::{post, web, App, HttpRequest, HttpServer, Responder};
use indexer_implementation::IndexerImplementation;
use kuchiki::traits::TendrilSink;
use lib::lib::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Mutex;

pub trait Indexer {
    //too many args?
    fn insert(
        &mut self,
        words: &[String],
        url: &str,
        title: &Option<String>,
        description: &Option<String>,
        language: &Option<String>,
        content: &str,
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

    serve_http_endpoint("0.0.0.0", 8080).await
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
            .service(add_resource)
            .service(
                web::resource(["/api/search", "/api/search/", "/api/search/{query}"]).to(search),
            )
            .service(
                actix_web_lab::web::spa()
                    .index_file("./frontend/dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./frontend/dist")
                    .finish(),
            ) //TODO: maybe separate gui backend from api?
    })
    .bind((address, port))?
    .run()
    .await
}

//TODO: sufficiently simmilar word in search (algorithm)
#[post("/api/resource")]
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
    let meta_selector = scraper::Selector::parse("meta").unwrap();
    let html_selector = scraper::Selector::parse("html").unwrap();

    let page_title: Option<String> = match document
        .select(&title_selector)
        .map(|e| e.inner_html())
        .take(1)
        .collect::<String>()
    {
        s if s.is_empty() => None,
        string => Some(string),
    };

    let page_description: Option<String> = match document
        .select(&meta_selector)
        .filter(|e| e.value().attr("name") == Some("description"))
        .filter_map(|e| e.value().attr("content"))
        .take(1)
        .collect::<String>()
    {
        s if s.is_empty() => None,
        string => Some(string),
    };

    //TODO: rewrite with if let else
    let page_language: Option<String> = match document
        .select(&html_selector)
        .filter_map(|e| e.value().attr("lang"))
        .take(1)
        .collect::<String>()
    {
        s if s.is_empty() => None,
        string => Some(string),
    };

    //and for each changed content word we add it to the db (word -> list.append(url))
    let mut indexer = data.indexer.lock().unwrap();
    let _ = indexer.insert(
        &fixed_words,
        &resource.url,
        &page_title,
        &page_description,
        &page_language,
        &resource.content,
    );

    //TODO: ADD LANG? EN in meta tag (frontend)
    //Now what to do, global lang?, per index lang?, website lang?
    //TODO: max number of results in query

    println!("Added resource: {:?}", indexer.num_of_words());

    format!("{resource:?}")
}

#[derive(Debug, Deserialize)]
struct OptSearchPath {
    query: Option<String>,
}

async fn search(
    _req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<OptSearchPath>,
) -> impl Responder {
    let indexer = data.indexer.lock().unwrap();

    let query = match &path.query {
        Some(query) => query,
        None => return "[]".to_string(),
    };

    println!("Query: {:?}", query);

    let results = indexer.search(query);
    //+is lowercase search good (we turn ascii lowercase, what do we do with inserting)

    serde_json::to_string(&results.unwrap()).unwrap()
}
