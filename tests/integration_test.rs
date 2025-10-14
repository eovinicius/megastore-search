use megastore_search::{Indexer, SearchEngine};
use megastore_search::indexer::Product;

#[test]
fn basic_search_returns_expected() {
    let mut idx = Indexer::new();

    idx.add_product(Product {
        id: 10,
        name: "Camisa Polo Azul".to_string(),
        brand: Some("ClothCo".to_string()),
        categories: vec!["vestuário".to_string()],
        attrs: Default::default(),
    });

    idx.add_product(Product {
        id: 11,
        name: "Camisa Polo Vermelha".to_string(),
        brand: Some("ClothCo".to_string()),
        categories: vec!["vestuário".to_string()],
        attrs: Default::default(),
    });

    let engine = SearchEngine::new(&idx);
    let res = engine.search("camisa polo", 5).unwrap();
    assert!(res.contains(&10));
    assert!(res.contains(&11));
}
