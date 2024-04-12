use fastembed::TextEmbedding;
use crate::embedding_strategy::entity_strategy::EntityStrategy;
use crate::graph_db::GraphDbFunc;
use crate::vector_db::VectorDbFunc;
use crate::vectorization_service::VectorizationFunc;

mod subgraph_strategy;
mod entity_strategy;
mod triple_strategy;

pub trait StrategyFunc {

    async fn load(&self);

    fn query(q: String) -> String;

}

pub struct StrategyBuilder<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc> {
    graph: G,
    vectorize: V,
    vector_db: VDB
}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc> StrategyBuilder<G, V, VDB> {
    pub fn new() -> StrategyBuilder<G, V, VDB> {
        StrategyBuilder::default()
    }

    pub fn graph(mut self, graph: impl GraphDbFunc) -> StrategyBuilder<G, V, VDB> {
        self.graph = graph;
        self
    }

    pub fn vectorize(mut self, vectorize: impl VectorizationFunc) -> StrategyBuilder<G, V, VDB> {
        self.vectorize = vectorize;
        self
    }

    pub fn vector_db(mut self, vector_db: impl VectorDbFunc) -> StrategyBuilder<G, V, VDB> {
        self.vector_db = vector_db;
        self
    }

    pub fn build_entity_strategy(self) -> EntityStrategy<G, V, VDB> {
        EntityStrategy::new(self.graph, self.vectorize, self.vector_db)
    }

    pub fn build_subgraph_strategy() {

    }

    pub fn build_triple_strategy() {

    }

}
