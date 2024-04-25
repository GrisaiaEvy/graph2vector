use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use neo4rs::{BoltList, BoltNode, BoltType, Graph, Node, query, Relation};
use crate::graph_db::{EdgeData, GraphDbFunc, GraphSchema, NodeData, SPO};

pub struct Neo4j {
    graph: Graph,
    db_name: String,
}

impl Neo4j {

    fn parse_bolt_node(n: BoltNode) -> (String, String) {
        let id = n.id.value.to_string();
        let first_label = n.labels.value.get(0)
            .map(|label| label.to_string())
            .unwrap_or_else(|| "none".to_string());
        (id, first_label)
    }

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

    async fn connect(params: Self::ConnParams) -> Self {
        let host = params.host;
        let port: u16 = params.port;
        let user = params.user;
        let pwd = params.pwd;

        let uri = format!("{host}:{port}");

        let graph = Graph::new(uri, user, pwd).await.unwrap();

        Neo4j {graph, db_name: params.db_name}
    }

    async fn vertexes(&self) -> Vec<NodeData> {
        let mut nodes: Vec<NodeData> = Vec::new();
        let mut result =
            self.graph.execute_on( self.db_name.as_str(), query("match (n) return n")).await.unwrap();
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
    }

    async fn edges(&self) -> Vec<EdgeData> {
        let mut edges: Vec<EdgeData> = Vec::new();
        let mut result =
            self.graph.execute_on(self.db_name.as_str(), query("match ()-[e]->() return e")).await.unwrap();

        while let Ok(Some(row)) = result.next().await {
            let mut edge_data = EdgeData::default();
            let relation: Relation = row.get("e").unwrap();
            edge_data.start_node_id = relation.start_node_id().to_string();
            edge_data.end_node_id = relation.end_node_id().to_string();
            edge_data.typ = relation.typ().to_string();
            edges.push(edge_data);
        }
        edges
    }

    async fn graph_schema(&self) -> GraphSchema {
        let mut result =
            self.graph.execute_on(self.db_name.as_str(), query("call db.schema.visualization()")).await.expect("query schema error");

        let mut spo_list = vec![];
        while let Ok(Some(row)) = result.next().await {
            let mut nodes: BoltList = row.get("nodes").expect("fetch nodes failed");
            // 转为map
            let mut id_node: HashMap<String, String> = HashMap::new();
            for n in nodes.into_iter() {
                match n {
                    BoltType::Node(val)=> {
                        let (id, label) = Self::parse_bolt_node(val);
                        id_node.insert(id, label);
                    }
                    _ => continue
                }
            }
            let relationships: BoltList = row.get("relationships").expect("fetch r\
            elations failed");
            for r in relationships.into_iter() {
                match r {
                    BoltType::Relation(val) => {
                        let end_node_id = val.end_node_id.value.to_string();
                        let start_node_id = val.start_node_id.value.to_string();
                        if id_node.contains_key(&start_node_id) && id_node.contains_key(&end_node_id) {
                            let a: SPO = (id_node.get(&start_node_id).unwrap().clone(), val.typ.value,
                                          id_node.get(&end_node_id).unwrap().clone());
                            spo_list.push(a);
                        }
                    }
                    _ => continue
                }
            }
        }
        GraphSchema { spo_list }
    }

    async fn subgraph(&self, id: &str) -> impl Future<Output=Result<String, Box<dyn Error>>> {
        let cypher = "MATCH ()<-[r]->(m) WHERE ID(n) = {id} RETURN relations r,m".replace("{id}", id);
        let mut result =
            self.graph.execute_on(self.db_name.as_str(), query(cypher.as_str())).await?;

    }
}
