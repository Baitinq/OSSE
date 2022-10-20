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

        let (_content, crawled_urls) = crawl_url(url.as_str());

        //println!("Content: {:?}", _content);
        println!("Next urls: {:?}", crawled_urls);

        //push content to index

        for url in crawled_urls {
            crawling_queue.push(url);
        }
    }
}

//takes url, returns content and list of urls
fn crawl_url(url: &str) -> (String, Vec<String>) {
    //return result
    let url = "https://".to_owned() + url;

    println!("Crawling {:?}", url);

    let response_res = reqwest::blocking::get(url);
    if response_res.is_err() {
        return (String::from(""), Vec::<String>::new());
    }
    let response_text_res = response_res.unwrap().text();
    if response_text_res.is_err() {
        return (String::from(""), Vec::<String>::new());
    }

    let response_text = response_text_res.unwrap();
    let document = scraper::Html::parse_document(response_text.as_str());

    let link_selector = scraper::Selector::parse("a").unwrap();
    let next_urls = document
        .select(&link_selector)
        .filter_map(|link| link.value().attr("href"))
        .map(String::from)
        .collect();

    //todo: filter urls that point to bad stuff? or we do that at the beggining of craw_url. we probs need to return result

    (response_text, next_urls)
}
