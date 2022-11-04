use lib::lib::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct IndexerImplementation {
    pub database: HashMap<String, HashSet<IndexedResource>>,
}

impl IndexerImplementation {
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
        }
    }

    fn search_word_in_db(&self, word: String) -> Option<&HashSet<IndexedResource>> {
        self.database.get(&word)
    }

    fn calculate_word_priority(word: &str, _html_site: &str, words: &[String]) -> u32 {
        //TODO: priorize lower levels of url, priorize word in url/title/description or main?

        //atm priority is just the number of occurences in the site.
        words.iter().filter(|w| *w == word).count() as u32
    }
}

impl crate::Indexer for IndexerImplementation {
    fn insert(
        &mut self,
        words: &[String],
        url: &str,
        title: Option<String>,
        description: Option<String>,
        content: &str,
    ) -> Result<(), String> {
        for word in words {
            let resource_to_add = IndexedResource {
                url: url.to_string(),
                priority: Self::calculate_word_priority(word, content, words),
                word: Arc::new(word.to_string()),
                title: title.as_ref().map(String::from),
                description: description.as_ref().map(String::from),
            };

            match self.database.get_mut(word) {
                Some(resources) => _ = resources.insert(resource_to_add),
                None => {
                    _ = self
                        .database
                        .insert(word.to_string(), HashSet::from([resource_to_add]))
                }
            }
        }

        Ok(())
    }

    fn search(&self, term: &str) -> Result<HashSet<IndexedResource>, String> {
        let query: Vec<&str> = term.split(' ').collect();

        //percentage of valid words
        let mut valid_results: Option<HashSet<IndexedResource>> = None;
        for w in query {
            //Normalise queries to lowercase
            let w = w.to_ascii_lowercase();

            let curr_word_results = match self.search_word_in_db(w.to_string()) {
                None => return Ok(HashSet::new()), //I dont really like this
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

        Ok(valid_results.unwrap())
    }

    fn num_of_words(&self) -> usize {
        self.database.len()
    }
}
