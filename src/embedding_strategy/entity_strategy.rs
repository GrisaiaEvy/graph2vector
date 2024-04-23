use log::debug;
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
}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc, M: LLM> StrategyFunc for EntityStrategy<G, V, VDB, M>  {
    async fn load_data(&self) {
        let x = self.graph.vertexes().await;
        if x.is_empty() {
            panic!("Data is empty!")
        }
        let mut cnt = 0;
        // tag p1=v1 p2=v2 ...
        for x in x.into_iter() {
            let mut s = String::new();
            s.push_str(x.tag.as_str());
            s.push(' ');
            for (k, v) in x.properties {
                s.push_str(k.as_str());
                s.push('=');
                s.push_str(v.as_str());
                s.push(' ');
            }

            let embedding = self.vectorize.vectorize(&s).await;

            self.vector_db.insert(x.id, s, embedding, String::new()).await;
            cnt += 1;
            debug!("插入了{}条数据", cnt);
        }
    }


    async fn build_query_context(&self, input: &str) -> String {
        let graph_schema = self.graph.graph_schema().await;
        let graph_schema_str = graph_schema.format();

        let extract_prompt = Self::extract_prompt(graph_schema_str.as_str());
        debug!("extract_prompt：{}", extract_prompt);

        let result = self.llm.completion(extract_prompt.as_str(), input).await;
        let entity_list: Vec<String> = serde_json::from_str(result.as_str()).expect("format result failed");
        debug!("extract entities [{:?}]", entity_list);

        let mut context = String::new();
        for x in entity_list.into_iter() {
            let vec = self.vectorize.vectorize(input).await;
            self.vector_db.search(vec, 6).await;
        }
        Self::user_prompt(graph_schema_str.as_str(), context.as_str(), input)
    }

    fn system_prompt(&self) -> String {
        String::from("Please follow this schema and extract entity form users query")
    }

    async fn query(&self, input: &str) {
        let user_prompt = self.build_query_context(input).await;
        let system_prompt = self.system_prompt();
        self.llm.stream_completion_cmd(system_prompt.as_str(), user_prompt.as_str()).await;
    }
}