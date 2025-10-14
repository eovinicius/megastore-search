use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};
use deunicode::deunicode;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub brand: Option<String>,
    pub categories: Vec<String>,
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Indexer {
    pub products: HashMap<u64, Product>,
    pub inverted: HashMap<String, HashSet<u64>>,
}

impl Indexer {
    pub fn new() -> Self { Self::default() }

    pub fn add_product(&mut self, p: Product) {
        let id = p.id;
        let mut tokens = tokenize(&p.name);
        if let Some(b) = &p.brand { tokens.extend(tokenize(b)); }
        for c in &p.categories { tokens.extend(tokenize(c)); }

        for t in tokens {
            self.inverted.entry(t).or_default().insert(id);
        }
        self.products.insert(id, p);
    }

    pub fn get_product(&self, id: &u64) -> Option<&Product> {
        self.products.get(id)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let bytes = bincode::serialize(self)?;
        let mut f = File::create(path)?;
        f.write_all(&bytes)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let idx: Indexer = bincode::deserialize(&buf)?;
        Ok(idx)
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
        
    let t = deunicode(text);
    t
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}
