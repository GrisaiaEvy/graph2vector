use std::collections::HashMap;
use std::error::Error;
use log::{debug, info};
use crate::config::CONF;
use crate::embedding_strategy::StrategyFunc;
use crate::graph_db::GraphDbFunc;
use crate::llm::LLM;
use crate::vector_db::VectorDbFunc;
use crate::vectorization_service::VectorizationFunc;

pub struct EntityStrategy<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc, M: LLM> {
    graph: G,
    vectorize: V,
    vector_db: VDB,
    llm: M
}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc, M: LLM>  EntityStrategy<G, V, VDB, M> {
    pub fn new(graph: G, vectorize: V, vector_db: VDB, llm: M) -> EntityStrategy<G, V, VDB, M> {
        EntityStrategy {
            graph,
            vectorize,
            vector_db,
            llm
        }
    }

    fn extract_prompt(graph_schema_str: &str) -> String {
        let extract_prompt = CONF.get_string("entity_extract_prompt").expect("failed get extract prompt");
        extract_prompt.replace("{schema}", graph_schema_str)
    }

    fn user_prompt(graph_schema_str: &str, context: &str, question: &str) -> String {
        let query_prompt = CONF.get_string("entity_user_prompt").expect("failed get user prompt");
        query_prompt.replace("{schema}", graph_schema_str).replace("{context}", context).replace("{question}", question)
    }

    async fn subgraph_prompt(&self, start_node_id: &str, start_node_content: &str) -> Result<String, Box<dyn Error>> {
        let subgraph_list = self.graph.subgraph(start_node_id).await?;
        let mut prompt = String::new();

        let mut hash_map: HashMap<String, String> = HashMap::new();
        let vec_node = subgraph_list.0;
        for n in vec_node.iter() {
            if !n.id.is_empty() {
                hash_map.insert(n.id.clone(), n.format());
            }
        }
        let vec_edge = subgraph_list.1;
        for e in vec_edge.into_iter() {
            if e.start_node_id == start_node_id {
                if let Some(r) = hash_map.get(&e.end_node_id) {
                    prompt.push_str(format!("{} -> {} -> {}", &start_node_content, &e.typ, r).as_str());
                }
            } else if e.end_node_id == start_node_id {
                if let Some(r) = hash_map.get(&e.start_node_id) {
                    prompt.push_str(format!("{} -> {} -> {}", r, &e.typ, &start_node_content).as_str());
                }
            } else {
                continue;
            }
            prompt.push_str("\n");
        }
        Ok(prompt)
    }
}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc, M: LLM> StrategyFunc for EntityStrategy<G, V, VDB, M>  {
    async fn load_data(&self) -> Result<(), Box<dyn Error>> {
        let x = self.graph.vertexes().await;
        if x.is_empty() {
            panic!("Data is empty!")
        }
        let mut cnt = 0;

        for x in x.iter() {
            let s = x.format();
            let embedding = self.vectorize.vectorize(&s).await?;

            self.vector_db.insert(x.id.clone(), s, embedding, String::new()).await?;
            cnt += 1;
            debug!("Insert data count: {}", cnt);
        }
        Ok(())
    }


    async fn build_query_context(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let graph_schema = self.graph.graph_schema().await;
        let graph_schema_str = graph_schema.format();

        let extract_prompt = Self::extract_prompt(graph_schema_str.as_str());
        debug!("extract_prompt：{}", extract_prompt);

        let result = self.llm.completion(extract_prompt.as_str(), input).await?;
        let entity_list: Vec<String> = serde_json::from_str(result.as_str())?;
        debug!("extract entities [{:?}]", entity_list);

        let mut context = String::new();
        for x in entity_list.into_iter() {
            let vec = self.vectorize.vectorize(input).await?;
            let vector_search_results =
                self.vector_db.search(vec, CONF.get("top_k").unwrap_or(1)).await?;
            for x in vector_search_results.iter() {
                context.push_str(&self.subgraph_prompt(x.id.as_str(), x.content.as_str()).await?);
            }
        }
        Ok(Self::user_prompt(graph_schema_str.as_str(), context.as_str(), input))
    }

    fn system_prompt(&self) -> Result<String, Box<dyn Error>> {
        Ok(CONF.get_string("system_prompt")?)
    }

    async fn query(&self, input: &str) -> Result<(), Box<dyn Error>> {
        let system_prompt = self.system_prompt()?;
        let user_prompt = self.build_query_context(input).await?;
        info!("user prompt: {}", user_prompt);
        self.llm.stream_completion_cmd(system_prompt.as_str(), user_prompt.as_str()).await;
        Ok(())
    }
}