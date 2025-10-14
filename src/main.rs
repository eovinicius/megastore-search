use megastore_search::{Indexer, SearchEngine};
use megastore_search::indexer::Product;

fn main() {
    let mut idx = Indexer::new();

    idx.add_product(Product {
        id: 1,
        name: "Smartphone X Alpha".to_string(),
        brand: Some("MegaBrand".to_string()),
        categories: vec!["eletrônicos".to_string(), "smartphones".to_string()],
        attrs: Default::default(),
    });

    idx.add_product(Product {
        id: 2,
        name: "Fone de Ouvido Wireless".to_string(),
        brand: Some("SoundCorp".to_string()),
        categories: vec!["eletrônicos".to_string(), "audio".to_string()],
        attrs: Default::default(),
    });

    let tmp = std::env::temp_dir().join("megastore_index.bin");
    if let Err(e) = idx.save(&tmp) {
        eprintln!("Falha ao salvar índice: {}", e);
        return;
    }

    let loaded = match Indexer::load(&tmp) {
        Ok(i) => i,
        Err(e) => { eprintln!("Falha ao carregar índice: {}", e); return; }
    };

    let engine = SearchEngine::new(&loaded);
    let res = engine.search("smartphone", 10).unwrap();
    println!("Resultado IDs após reload: {:?}", res);
}
