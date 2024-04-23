use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};
use crate::vectorization_service::VectorizationFunc;

pub struct FastEmbed {
    model: TextEmbedding
}

impl FastEmbed {

    pub fn new() -> FastEmbed {
        let model = TextEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::BGESmallZHV15,
            show_download_progress: true,
            ..Default::default()
        }).unwrap();
        FastEmbed {
            model
        }
    }
}

impl VectorizationFunc for FastEmbed {


    async fn vectorize(&self, sentences: &str) -> Embedding {
        let vec1 = vec![sentences];
        let batch
            = self.vectorize_batch(vec1).await;
        batch.into_iter().next().unwrap()
    }

    async fn vectorize_batch(&self, sentences: Vec<&str>) -> Vec<Embedding> {
        self.model.embed(sentences, None).expect("Vectorize failed!")
    }
}