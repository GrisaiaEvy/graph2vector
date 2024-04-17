use std::future::Future;
use fastembed::Embedding;
use crate::graph_db::NodeData;

pub mod vector_cache;
pub mod fastembed_service;

pub trait VectorizationFunc {

    async fn vectorize(&self, sentences: String) -> Embedding;

    async fn vectorize_batch(&self, sentences: Vec<String>) -> Vec<Embedding>;

}