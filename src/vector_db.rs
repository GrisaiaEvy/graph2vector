use fastembed::Embedding;

pub mod milvus_vector_db;

pub trait VectorDbFunc {

    const SCHEMA_NAME: &'static str = "graph_data";

    const ID_FIELD: &'static str = "embed";

    const VECTOR_FIELD: &'static str = "vector";

    const CONTENT_FIELD: &'static str = "content";

    const METADATA_FIELD: &'static str = "metadata";

    async fn insert(&self, id: String, content: String, vector: Vec<f32>, metadata: String);

    async fn search(&self);

}