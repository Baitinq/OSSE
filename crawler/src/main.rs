use itertools::Itertools;

fn main() {
    println!("Hello, world! Im the crawler!");

    let root_urls = include_str!("../top-1000-websites.txt");
    let root_urls = root_urls.split('\n').collect();

    crawler(root_urls);
}

//takes list of strings - multithread here?
fn crawler(root_urls: Vec<&str>) {
    println!("Starting to crawl!");

    //add root urls to queue - TODO: max q size
    let crawling_queue: blockingqueue::BlockingQueue<String> = blockingqueue::BlockingQueue::new();
    for url in root_urls {
        crawling_queue.push(String::from(url));
    }

    //and start crawling
    //FIXME: Async!
    loop {
        //blocks
        let url = crawling_queue.pop();

        let crawl_res = crawl_url(url.as_str());
        if crawl_res.is_err() {
            println!("Error crawling {}", url);
            continue;
        }

        let (_content, crawled_urls) = crawl_res.unwrap();

        //println!("Content: {:?}", _content);
        println!("Next urls: {:?}", crawled_urls);

        //push content to index
        _ = push_crawl_entry_to_indexer(url, _content);

        for url in crawled_urls {
            crawling_queue.push(url);
        }
    }
}

fn crawl_url(url: &str) -> Result<(String, Vec<String>), String> {
    let url = "https://".to_owned() + url;

    println!("Crawling {:?}", url);

    let response_res = reqwest::blocking::get(&url);
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

fn push_crawl_entry_to_indexer(_url: String, _content: String) -> Result<(), ()> {
    Ok(())
}
