pub mod lib {

    use serde::{Serialize,Deserialize};
    use std::sync::Arc;
    use std::hash::{Hash, Hasher};
    use std::cmp::Ordering;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CrawledResource {
        pub url: String,
        pub content: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IndexedResource {
        pub url: String,
        pub title: String,
        pub description: String,
        pub priority: u32,
        pub word: Arc<String>,
    }

    //We implement PartialEq, Eq and Hash to ignore the priority field.
    impl PartialEq for IndexedResource {
        fn eq(&self, other: &Self) -> bool {
            self.url == other.url && self.word == other.word
        }
    }
    impl Eq for IndexedResource {}

    impl PartialOrd for IndexedResource {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    //Reverse ordering as priority: 1 is less than priority: 2
    impl Ord for IndexedResource {
        fn cmp(&self, other: &Self) -> Ordering {
            self.priority.cmp(&other.priority).reverse()
        }
    }

    impl Hash for IndexedResource {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.url.hash(state);
            self.word.hash(state);
        }
    }

}