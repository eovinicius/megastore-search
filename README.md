# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição

Projeto acadêmico que implementa um sistema de busca otimizado para o catálogo de produtos da MegaStore. O objetivo é indexar produtos de forma eficiente e permitir buscas rápidas por nome, marca, categoria e outros campos, com foco em estruturas baseadas em tabelas hash.

## Objetivos

- Implementar um índice invertido eficiente.
- Suportar buscas relevantes em grande escala com algoritmos de ranking.
- Fornecer persistência do índice e testes que garantam correção.
- Documentar arquitetura, decisões e instruções de uso.

## Tecnologias e crates usados

- Rust (edition 2021)
- serde / serde_json
- bincode (persistência binária)
- deunicode (normalização/transliteração)
- anyhow (erros)
- tempfile (dev-dependency para testes)

## Como rodar

Pré-requisitos: Rust toolchain (rustup + cargo).

No Windows PowerShell:

```powershell
# Compilar
cargo build --release

# Rodar testes
cargo test

# Executar exemplo (main.rs demonstra save/load)
cargo run --release
```

## Exemplos de uso

- Consulta simples: `smartphone`
- Consulta multi-token: `camisa polo`

## Arquitetura e algoritmos

- Indexação: índice invertido `HashMap<String, HashSet<u64>>` (tokens -> ids de produtos).
- Tokenização: transliteração com `deunicode`, split por não-alfanuméricos e casefolding.
- Busca: coleta candidatos a partir das posting lists e rankeamento por TF‑IDF (implementado). Opção para AND estrito pode ser adicionada.
- Persistência: `Indexer::save` / `Indexer::load` via `bincode`.

Foco em tabelas hash: a estrutura central é `HashMap` para acesso O(1) por token.

## Considerações sobre desempenho e escalabilidade

- Memory-bound: índices em memória crescem com o catálogo; usar serialização em disco ou motores dedicados para catálogos muito grandes.
- Concorrência: em produção, encapsular índice em `Arc<RwLock<..>>` ou usar `dashmap`.
- Ranking: BM25 é recomendado para produção. TF‑IDF aqui é uma implementação simples.

## Testes

- `cargo test` executa suite de testes unitários e de integração.

## Contribuições

Pull requests bem-vindos. Abra issue descrevendo o problema antes de PR grande.

## Licença

MIT

## Gerar PDF da documentação

Há um relatório detalhado em `docs/report_detailed.md`. Para gerar o PDF localmente (Windows PowerShell):

```powershell
# Gere o PDF (requer pandoc e LaTeX/xelatex)
.\scripts\generate_pdf.ps1
```

Isso gerará `docs/report_detailed.pdf`.
