use std::collections::{HashSet, HashMap};
use crate::indexer::{Indexer, tokenize};
use anyhow::Result;

pub struct SearchEngine<'a> {
    pub indexer: &'a Indexer,
}

impl<'a> SearchEngine<'a> {
    pub fn new(indexer: &'a Indexer) -> Self { Self { indexer } }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<u64>> {
        let tokens = tokenize(query);
    if tokens.is_empty() { return Ok(vec![]); }
        let mut candidate_ids: HashSet<u64> = HashSet::new();
        for t in &tokens {
            if let Some(posting) = self.indexer.inverted.get(t) {
                for id in posting.iter() { candidate_ids.insert(*id); }
            }
        }

        if candidate_ids.is_empty() { return Ok(vec![]); }

        
        let n_docs = self.indexer.products.len() as f64;
        let mut scored: Vec<(u64, f64)> = Vec::new();

        for doc_id in candidate_ids.into_iter() {
            let mut score = 0f64;
            
            let p = self.indexer.products.get(&doc_id).unwrap();
            let mut tf_counts: HashMap<String, usize> = HashMap::new();

            for token in tokenize(&p.name) { *tf_counts.entry(token).or_insert(0) += 1; }
            if let Some(b) = &p.brand { for token in tokenize(b) { *tf_counts.entry(token).or_insert(0) += 1; } }
            for c in &p.categories { for token in tokenize(c) { *tf_counts.entry(token).or_insert(0) += 1; } }

            for q in &tokens {
                let df = self.indexer.inverted.get(q).map(|s| s.len()).unwrap_or(0) as f64;
                if df == 0f64 { continue; }
                let idf = (n_docs / df).ln() + 1.0;
                let tf = *tf_counts.get(q.as_str()).unwrap_or(&0) as f64;
                score += tf * idf;
            }

            scored.push((doc_id, score));
        }

        scored.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let res: Vec<u64> = scored.into_iter().map(|(id, _)| id).take(limit).collect();
        Ok(res)
    }
}
