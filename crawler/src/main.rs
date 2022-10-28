use itertools::Itertools;
use rand::seq::IteratorRandom;
use reqwest::{Client, Response, StatusCode};
use serde::Serialize;
use url::Url;

#[tokio::main]
async fn main() {
    println!("Hello, world! Im the crawler!");

    let root_urls = include_str!("../top-1000-websites.txt");
    let root_urls = root_urls.split('\n').collect();

    let http_client = reqwest::Client::new();

    crawler(http_client, root_urls).await;
}

//TODO: crawling depth? - async http client
async fn crawler(http_client: Client, root_urls: Vec<&str>) {
    dbg!("Starting to crawl!");

    //add root urls to queue - TODO: max q size
    let (tx_crawling_queue, rx_crawling_queue) = async_channel::bounded::<String>(2222);
    for url in root_urls {
        tx_crawling_queue.send(url.to_string()).await.unwrap();
    }

    //and start crawling
    loop {
        //even if we clone, the underlying queue implementation is still shared
        let tx_crawling_queue = tx_crawling_queue.clone();
        let rx_crawling_queue = rx_crawling_queue.clone();
        //blocks - we move it up here as to at least block for next url and not endesly spawn tasks
        let url = rx_crawling_queue.recv().await.unwrap();
        let http_client = http_client.clone();
        tokio::spawn(async move {
            let (content, crawled_urls) = match crawl_url(&http_client, url.as_str()).await {
                Err(e) => {
                    println!("Error crawling ({}): {}", url, e);
                    return;
                }
                Ok(result) => result,
            };

            //DONT FORGET ENUMS
            //CAN WE DO UNWRAP OR RETURN or lambda
            //HOW TF DOES CRAWLER WORK. DOESNT QUEUE FILL. LOTS OF WAITING THINGS??

            //dbg!("Content: {:?}", &content);
            dbg!("Next urls: {:?}", &crawled_urls);

            //push content to index
            let indexer_response = match push_crawl_entry_to_indexer(
                &http_client,
                "http://127.0.0.1:4444/resource".to_string(),
                url,
                content,
            )
            .await
            {
                Err(e) => {
                    println!("{e}");
                    return;
                }
                Ok(res) => res.text().await,
            };

            dbg!("Pushed to indexer {:?}", &indexer_response);

            for url in crawled_urls {
                tx_crawling_queue.send(url).await.unwrap();
            }
        });
    }
}

async fn crawl_url(http_client: &Client, url: &str) -> Result<(String, Vec<String>), String> {
    dbg!("Crawling {:?}", url);

    let url = Url::parse(url).unwrap();

    let response_text = match http_client.get(url.as_str()).send().await {
        Ok(text_res) if text_res.status() == StatusCode::OK => match text_res.text().await {
            Err(_) => {
                Err("Error unwrapping the fetched HTML's text (".to_owned() + url.as_str() + ")")
            }
            Ok(text) => Ok(text),
        },

        _ => Err("Error fetching ".to_owned() + url.as_str()),
    }?;

    let document = scraper::Html::parse_document(response_text.as_str());

    let valid_url = |check_url: &Url| match check_url {
        u if !(u.scheme() == "http" || u.scheme() == "https") => false,
        u if u.fragment().is_some() => false, //no # urls
        u if u.query().is_some() => false,    //no ? urls
        u if *u == url => false,              //no same url
        _ => true,
    };

    let link_selector = scraper::Selector::parse("a").unwrap();
    let next_urls = document
        .select(&link_selector)
        .filter_map(|link| link.value().attr("href"))
        .unique()
        .map(|u| url.join(u))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(valid_url)
        .map(String::from)
        .choose_multiple(&mut rand::thread_rng(), 2); //we shuffle as to minimise repeating links

    //normalise words somewhere
    //fuzzy? - iterate over keys
    //probs lots of places where we can borrow or not do stupid stuff
    //search for phrases?
    //http workings lagging behind crawler, what to do?
    //group responses and transmit them in an array of 10 or smth -> or maybe just lower q size!
    //use structs in database indexer
    //we need words priority or word list or smth (or in value of database show number of occurance or just val of importance of occurances)
    //i dont understand dbg! (how to print {})
    //is there empty urls?
    //user agent?

    println!("Returning next urls, {:?}", next_urls);
    Ok((response_text, next_urls))
}

async fn push_crawl_entry_to_indexer(
    http_client: &Client,
    indexer_url: String,
    url: String,
    content: String,
) -> Result<Response, String> {
    dbg!("Pushin to indexer");

    #[derive(Serialize, Debug)]
    struct Resource {
        url: String,
        content: String,
    }

    let request_body = Resource { url, content };

    match http_client
        .post(&indexer_url)
        .json(&request_body)
        .send()
        .await
    {
        Err(_) => Err(format!(
            "Error pushing the crawler to indexer! {:?}",
            &indexer_url
        )),
        Ok(response) => Ok(response),
    }
}
