pub mod lib {

    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CrawledResource {
        pub url: String,
        pub content: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IndexedResource {
        pub url: String,
        pub title: Option<String>,
        pub description: Option<String>,
        pub priority: u32,
        pub word: Arc<String>,
        pub language: Option<String>,
        //maybe in the future we need filetypes?
    }

    //We implement PartialEq, Eq and Hash to only care about the url field.
    impl PartialEq for IndexedResource {
        fn eq(&self, other: &Self) -> bool {
            self.url == other.url
        }
    }
    impl Eq for IndexedResource {}

    impl PartialOrd for IndexedResource {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    //Priority 1 is higher than priority 2
    impl Ord for IndexedResource {
        fn cmp(&self, other: &Self) -> Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl Hash for IndexedResource {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.url.hash(state)
        }
    }
}
