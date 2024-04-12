mod vector_cache;
mod fastembed_service;

pub trait VectorizationFunc {

    async fn vectorize(sentences: Vec<String>) -> Vec<f64>;

    async fn vectorize_batch(sentences: Vec<String>) -> Vec<f64>;

}