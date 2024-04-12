use fastembed::TextEmbedding;
use crate::vectorization_service::vector_cache::VectorCache;
use crate::vectorization_service::VectorizationFunc;

pub struct FastEmbed {
}

impl FastEmbed {

    pub fn new() -> FastEmbed {
        FastEmbed {}
    }
}

impl VectorizationFunc for FastEmbed {


    async fn vectorize(sentences: Vec<String>) -> Vec<f64> {
        let model = TextEmbedding::try_new(Default::default()).unwrap();

        let doc = vec!["hello world", "thats wired"];
        let embeddings = model.embed(doc, None).unwrap();
        print!("embedding: {:?}", embeddings)

    }

    async fn vectorize_batch(sentences: Vec<String>) -> Vec<f64> {
        todo!()
    }
}