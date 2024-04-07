use fastembed::TextEmbedding;

mod abs_vectorize_strategy;
mod subgraph_strategy;
mod entity_strategy;
mod triple_strategy;

fn vectorize() {
    let model = TextEmbedding::try_new(Default::default()).unwrap();

    let doc = vec!["hello world", "thats wired"];
    let embeddings = model.embed(doc, None).unwrap();
    print!("embedding: {:?}", embeddings);
}