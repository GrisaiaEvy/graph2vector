use std::future::Future;
use fastembed::Embedding;

pub mod milvus_vector_db;

pub trait VectorDbFunc {

    const SCHEMA_NAME: &'static str = "knowledge_graph_data";

    const ID_FIELD: &'static str = "id";

    const VECTOR_FIELD: &'static str = "vector";

    const CONTENT_FIELD: &'static str = "content";

    const METADATA_FIELD: &'static str = "metadata";

    fn insert(&self, id: String, content: String, vector: Vec<f32>, metadata: String) -> impl Future<Output = ()>;

    fn search(&self, vector: Vec<f32>, top_k: i32) -> impl Future<Output = String>;

}