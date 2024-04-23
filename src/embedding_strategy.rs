use std::future::Future;
use std::io;
use std::io::Write;
use log::debug;
use crate::embedding_strategy::entity_strategy::EntityStrategy;
use crate::graph_db::GraphDbFunc;
use crate::llm::LLM;
use crate::vector_db::VectorDbFunc;
use crate::vectorization_service::VectorizationFunc;

mod entity_strategy;

pub trait StrategyFunc {

    fn load_data(&self) -> impl Future<Output = ()>;

    fn build_query_context(&self, input: &str) -> impl Future<Output = String>;

    fn system_prompt(&self) -> String;

    fn query(&self, input: &str) -> impl Future<Output = ()>;

    async fn launch_ai_cmd(&self) {
        loop {
            debug!("Please input your question：");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("read failed");
            let input = input.trim();

            self.query(input).await;

            if input == "quit" { break; }
        }
    }

}

pub struct StrategyBuilder<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc, M: LLM> {
    graph: G,
    vectorize: V,
    vector_db: VDB,
    llm: M
}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc, M: LLM> StrategyBuilder<G, V, VDB, M> {
    pub fn new(graph: G, vectorize: V, vector_db: VDB, llm: M) -> StrategyBuilder<G, V, VDB, M> {
        StrategyBuilder {
            graph,
            vectorize,
            vector_db,
            llm
        }
    }

    pub fn build_entity_strategy(self) -> EntityStrategy<G, V, VDB, M> {
        EntityStrategy::new(self.graph, self.vectorize, self.vector_db, self.llm)
    }

    pub fn build_subgraph_strategy() {

    }

    pub fn build_triple_strategy() {

    }

}
