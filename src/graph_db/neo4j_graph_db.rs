use neo4rs::{Graph, Node, query};
use tokio::time::Instant;
use crate::graph_db::GraphDbFunc;

pub struct Neo4j {
    graph: Graph,
    db_name: String
}

pub struct Neo4jParams {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pwd: String,
    pub db_name: String,
}

impl GraphDbFunc for Neo4j {

    type ConnParams = Neo4jParams;

    async fn connect(params: Self::ConnParams) -> Self where Self: Sized {
        let host = params.host;
        let port: u16 = params.port;
        let user = params.user;
        let pwd = params.pwd;

        let uri = format!("{host}:{port}");

        let graph = Graph::new(uri, user, pwd).await.unwrap();

        Neo4j {graph, db_name: params.db_name}
    }

    async fn vertexes(&self) {
        let graph = self.graph.clone();
        let db_name = self.db_name.clone();
        println!("异步执行前");

        let start_time = Instant::now();

        // 在异步块中等待异步任务的完成
        tokio::spawn(async move {
            let mut result =
                graph.execute_on(db_name.as_str(), query("match (n) return n")).await.unwrap();
            println!("开始查询数据");
            while let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("n").unwrap();
                println!("{:?}", node)
            }
        }).await.expect("Failed to await async task");

        let end_time = Instant::now();
        println!("异步执行后");
        println!("异步执行后，花费了{:?}", end_time - start_time);
    }

    fn edges(&self) {
        todo!()
    }
}
