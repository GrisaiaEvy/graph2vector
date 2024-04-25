use std::error::Error;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};
use milvus::proto::common::ErrorCode;
use crate::vectorization_service::VectorizationFunc;

pub struct FastEmbed {
    model: TextEmbedding
}

impl FastEmbed {

    pub fn new_zh() -> FastEmbed {
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


    async fn vectorize(&self, sentences: &str) -> Result<Embedding, Box<dyn Error>> {
        let vec1 = vec![sentences];
        let batch
            = self.vectorize_batch(vec1).await?;
        Ok(batch.into_iter().next().expect("Empty embedding list."))
    }

    async fn vectorize_batch(&self, sentences: Vec<&str>) -> Result<Vec<Embedding>, Box<dyn Error>> {
        Ok(self.model.embed(sentences, None)?)
    }
}