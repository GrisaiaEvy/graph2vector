use neo4rs::{BoltType, Graph, Node, query, Relation};
use tokio::time::Instant;
use crate::graph_db::{EdgeData, GraphDbFunc, NodeData};

pub struct Neo4j {
    graph: Graph,
    db_name: String
}

impl Neo4j {
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

    async fn vertexes(&self) -> Vec<NodeData> {
        let graph = self.graph.clone();
        let db_name = self.db_name.clone();
        tokio::spawn(async move {
            let mut nodes: Vec<NodeData> = Vec::new();
            let mut result =
                graph.execute_on(db_name.as_str(), query("match (n) return n")).await.unwrap();
            while let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("n").unwrap();
                let mut node_data = NodeData::default();
                node_data.id = node.id().to_string();
                let first_label = node.labels().get(0)
                    .map(|label| label.to_string())
                    .unwrap_or_else(|| "无标签".to_string());
                node_data.tag = first_label;

                for (_, key) in node.keys().into_iter().enumerate() {
                    let i = node.get::<BoltType>(key).expect("None property");
                    let mut p_str = String::new();
                    match i  {
                        BoltType::String(val) => {
                            p_str = val.value
                        }
                        BoltType::Integer(val) => {
                            p_str = val.value.to_string()
                        }
                        _ => continue
                    }
                    node_data.properties.insert(key.to_string(), p_str);
                }
                nodes.push(node_data);
            }
            nodes
        }).await.expect("Failed to await async task")
    }

    async fn edges(&self) -> Vec<EdgeData> {
        let graph = self.graph.clone();
        let db_name = self.db_name.clone();

        tokio::spawn(async move {
            let mut edges: Vec<EdgeData> = Vec::new();
            let mut result =
                graph.execute_on(db_name.as_str(), query("match ()-[e]->() return e")).await.unwrap();

            let mut edge_data = EdgeData::default();

            while let Ok(Some(row)) = result.next().await {
                let relation: Relation = row.get("e").unwrap();
                edge_data.start_node_id = relation.start_node_id().to_string();
                edge_data.end_node_id = relation.end_node_id().to_string();
                edge_data.typ = relation.typ().to_string();
                edges.push(edge_data);
            }
            edges
        }).await.expect("Query edge failed")
    }



}
