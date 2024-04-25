use std::error::Error;
use std::future::Future;
use std::io;
use std::io::Write;
use crate::embedding_strategy::entity_strategy::EntityStrategy;
use crate::graph_db::GraphDbFunc;
use crate::llm::LLM;
use crate::vector_db::VectorDbFunc;
use crate::vectorization_service::VectorizationFunc;

mod entity_strategy;

pub trait StrategyFunc {

    fn load_data(&self) -> impl Future<Output=Result<(), Box<dyn Error>>>;

    fn build_query_context(&self, input: &str) -> impl Future<Output = Result<String, Box<dyn Error>>>;

    fn system_prompt(&self) -> Result<String, Box<dyn Error>>;

    fn query(&self, input: &str) -> impl Future<Output = Result<(), Box<dyn Error>>>;

    async fn launch_ai_cmd(&self) {
        loop {
            println!("Please input your question:");

            if let Err(e) = io::stdout().flush() {
                eprintln!("Filed to flush stdout: {}", e);
                return;
            }

            let mut input = String::new();
            if let Err(e) = io::stdin().read_line(&mut input) {
                eprintln!("Failed to get input, please try again. Error: {}", e);
            }
            let input = input.trim();

            if input == "quit" || input == "exit" { break; }

            if let Err(e) = self.query(input).await {
                eprintln!("Query failed: {}", e);
            }
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
