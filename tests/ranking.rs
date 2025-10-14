use megastore_search::{Indexer, SearchEngine};
use megastore_search::indexer::Product;

#[test]
fn tf_idf_ranks_more_relevant_first() {
    let mut idx = Indexer::new();
    idx.add_product(Product {
        id: 1,
        name: "Camiseta vermelho".to_string(),
        brand: None,
        categories: vec!["vestuario".to_string()],
        attrs: Default::default(),
    });
    idx.add_product(Product {
        id: 2,
        name: "Camiseta vermelha vermelho vermelho".to_string(),
        brand: None,
        categories: vec!["vestuario".to_string()],
        attrs: Default::default(),
    });

    let engine = SearchEngine::new(&idx);
    let res = engine.search("vermelho", 10).unwrap();
    assert!(res.len() >= 2);
    assert_eq!(res[0], 2);
}
