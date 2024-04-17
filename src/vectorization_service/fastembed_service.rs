use fastembed::{Embedding, TextEmbedding};
use crate::vectorization_service::VectorizationFunc;

pub struct FastEmbed {
}

impl FastEmbed {

    pub fn new() -> FastEmbed {
        FastEmbed {}
    }
}

impl VectorizationFunc for FastEmbed {


    async fn vectorize(&self, sentences: String) -> Embedding {
        let vec1 = vec![sentences];
        let batch
            = self.vectorize_batch(vec1).await;
        batch.into_iter().next().unwrap()
    }

    async fn vectorize_batch(&self, sentences: Vec<String>) -> Vec<Embedding> {
        let model = TextEmbedding::try_new(Default::default()).unwrap();
        model.embed(sentences, None).expect("Vectorize failed!")
    }
}