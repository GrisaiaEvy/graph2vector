use crate::embedding_strategy::StrategyFunc;
use crate::graph_db::GraphDbFunc;
use crate::vector_db::VectorDbFunc;
use crate::vectorization_service::VectorizationFunc;

pub struct EntityStrategy<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc> {
    graph: G,
    vectorize: V,
    vector_db: VDB
}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc>  EntityStrategy<G, V, VDB> {

}

impl<G: GraphDbFunc, V: VectorizationFunc, VDB: VectorDbFunc>  StrategyFunc for EntityStrategy<G, V, VDB>  {
    async fn load(&self) {
        let x = self.graph.vertexes().await;
        if x.is_empty() {
            panic!("Data is empty!")
        }
        let mut s = String::new();
        // tag p1=v1 p2=v2 ...
        for x in x.into_iter() {
            s.push_str(x.tag.as_str());
            s.push(' ');
            for (k, v) in x.properties {
                s.push_str(k.as_str());
                s.push('=');
                s.push_str(v.as_str());
                s.push(' ');
            }

        }

        // 分批调用向量化

        // 分批插入数据库
        self.vectorize.

    }

    fn query(q: String) -> String {
        todo!()
    }
}