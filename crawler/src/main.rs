use itertools::Itertools;
use reqwest::blocking::{Client, Response};
use serde::Serialize;

#[tokio::main]
async fn main() {
    println!("Hello, world! Im the crawler!");

    let root_urls = include_str!("../top-1000-websites.txt");
    let root_urls = root_urls.split('\n').collect();

    let http_client = reqwest::blocking::Client::new();

    crawler(http_client, root_urls).await;
}

//TODO: crawling depth? - async http client
async fn crawler(http_client: Client, root_urls: Vec<&str>) {
    println!("Starting to crawl!");

    //add root urls to queue - TODO: max q size
    let crawling_queue: blockingqueue::BlockingQueue<String> = blockingqueue::BlockingQueue::new();
    root_urls
        .into_iter()
        .for_each(|u| crawling_queue.push(String::from(u)));

    //and start crawling
    loop {
        //even if we clone, the underlying queue implementation is still shared
        let crawling_queue = crawling_queue.clone();
        let http_client = http_client.clone();
        tokio::spawn(async move {
            //blocks
            let url = crawling_queue.pop();

            let crawl_res = crawl_url(&http_client, url.as_str()).await;
            if crawl_res.is_err() {
                println!("Error crawling {}", url);
                return;
            }

            let (content, crawled_urls) = crawl_res.unwrap();

            //println!("Content: {:?}", content);
            println!("Next urls: {:?}", crawled_urls);

            //push content to index
            let indexer_res = push_crawl_entry_to_indexer(
                &http_client,
                String::from("http://127.0.0.1:4444/resource"),
                url,
                content,
            )
            .await
            .unwrap()
            .text();

            println!("Pushed to indexer {:?}", &indexer_res);

            crawled_urls
                .into_iter()
                .for_each(|u| crawling_queue.push(u));
        });
    }
}

async fn crawl_url(http_client: &Client, url: &str) -> Result<(String, Vec<String>), String> {
    let url = "https://".to_owned() + url;

    println!("Crawling {:?}", url);

    let response_res = http_client.get(&url).send();
    if response_res.is_err() {
        return Err("Error fetching ".to_owned() + &url);
    }
    let response_text_res = response_res.unwrap().text();
    if response_text_res.is_err() {
        return Err("Error unwrapping the fetched HTML's text (".to_owned() + &url + ")");
    }

    let response_text = response_text_res.unwrap();
    let document = scraper::Html::parse_document(response_text.as_str());

    let link_selector = scraper::Selector::parse("a").unwrap();
    let next_urls = document
        .select(&link_selector)
        .filter_map(|link| link.value().attr("href"))
        .unique()
        .map(String::from)
        .collect();

    let fixup_urls = |us: Vec<String>| {
        us.into_iter()
            .map(|u| {
                //https://stackoverflow.com/questions/9646407/two-forward-slashes-in-a-url-src-href-attribute
                if u.starts_with("//") {
                    format!("https:{}", &u)
                } else if u.starts_with('/') {
                    format!("{}{}", &url, &u)
                } else {
                    u
                }
            })
            .collect()
    };

    let next_urls = fixup_urls(next_urls);

    Ok((response_text, next_urls))
}

async fn push_crawl_entry_to_indexer(
    http_client: &Client,
    indexer_url: String,
    url: String,
    content: String,
) -> Result<Response, String> {
    println!("Pushin to indexer");

    #[derive(Serialize, Debug)]
    struct Resource {
        url: String,
        content: String,
    }

    let request_body = Resource { url, content };

    let response_res = http_client.post(&indexer_url).json(&request_body).send();
    if response_res.is_err() {
        return Err(format!(
            "Error pushing the crawler to indexer! {:?}",
            &indexer_url
        ));
    }

    Ok(response_res.unwrap())
}
