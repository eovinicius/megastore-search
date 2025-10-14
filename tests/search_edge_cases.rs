use megastore_search::{Indexer, SearchEngine};
use megastore_search::indexer::Product;

#[test]
fn empty_query_returns_empty() {
    let idx = Indexer::new();
    let engine = SearchEngine::new(&idx);
    let res = engine.search("", 10).unwrap();
    assert!(res.is_empty());
}

#[test]
fn unknown_token_returns_empty() {
    let mut idx = Indexer::new();
    idx.add_product(Product { id: 1, name: "Produto A".to_string(), brand: None, categories: vec!["geral".to_string()], attrs: Default::default() });
    let engine = SearchEngine::new(&idx);
    let res = engine.search("qwertyuiop", 5).unwrap();
    assert!(res.is_empty());
}

#[test]
fn respects_limit_parameter() {
    let mut idx = Indexer::new();
    for i in 1..=10 {
        idx.add_product(Product { id: i, name: format!("Produto {}", i), brand: None, categories: vec!["geral".to_string()], attrs: Default::default() });
    }
    let engine = SearchEngine::new(&idx);
    let res = engine.search("produto", 3).unwrap();
    assert_eq!(res.len(), 3);
}
