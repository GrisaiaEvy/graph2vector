use std::error::Error;
use std::future::Future;
use fastembed::Embedding;

pub mod vector_cache;
pub mod fastembed_service;

pub trait VectorizationFunc {

    fn vectorize(&self, sentences: &str) -> impl Future<Output = Result<Embedding, Box<dyn Error>>>;

    fn vectorize_batch(&self, sentences: Vec<&str>) -> impl Future<Output = Result<Vec<Embedding>, Box<dyn Error>>>;

}