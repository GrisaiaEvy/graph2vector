use std::future::Future;
use fastembed::Embedding;
use crate::graph_db::NodeData;

pub mod vector_cache;
pub mod fastembed_service;

pub trait VectorizationFunc {

    fn vectorize(&self, sentences: &str) -> impl Future<Output = Embedding>;

    fn vectorize_batch(&self, sentences: Vec<&str>) -> impl Future<Output = Vec<Embedding>>;

}