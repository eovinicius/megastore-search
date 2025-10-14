use megastore_search::indexer::{tokenize, Indexer, Product};
use tempfile::NamedTempFile;

#[test]
fn tokenize_removes_accents_and_splits() {
    let s = "Camisa Polo Azul - Eletrônicos";
    let tokens = tokenize(s);
    assert!(tokens.contains(&"camisa".to_string()));
    assert!(tokens.contains(&"polo".to_string()));
    assert!(tokens.contains(&"azul".to_string()));
    assert!(tokens.contains(&"eletronicos".to_string()));
}

#[test]
fn save_and_load_indexer_roundtrip() {
    let mut idx = Indexer::new();
    idx.add_product(Product {
        id: 42,
        name: "Teste Produto".to_string(),
        brand: Some("Marca".to_string()),
        categories: vec!["geral".to_string()],
        attrs: Default::default(),
    });

    let tmp = NamedTempFile::new().expect("tempfile");
    let path = tmp.path();
    idx.save(path).expect("save");
    let loaded = Indexer::load(path).expect("load");
    assert!(loaded.get_product(&42).is_some());
}
