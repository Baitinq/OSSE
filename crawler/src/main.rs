use blockingqueue::*;

fn main() {
    println!("Hello, world! Im the crawler!");

    let root_urls = include_str!("../top-1000-websites.txt");
    let root_urls = root_urls.split('\n').collect();

    crawler(root_urls);
}

//takes list of strings - multithread here?
fn crawler(root_urls: Vec<&str>) {
    println!("Starting to crawl!");

    //add root urls to queue
    let crawling_queue: BlockingQueue<&str> = BlockingQueue::new();
    for url in root_urls {
        crawling_queue.push(url);
    }

    //and start crawling
    //FIXME: Async!
    loop {
        //blocks
        let url = crawling_queue.pop();

        let (content, crawled_urls) = crawl_url(url);

        //push content to index

        for url in crawled_urls {
            crawling_queue.push(url);
        }
    }
}

//takes url, returns content and list of urls
fn crawl_url(url: &str) -> (&str, Vec<&str>) {
    println!("Crawling {:?}", "https://".to_owned() + url);

    ("", vec![])
}
