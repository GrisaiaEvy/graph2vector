use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use neo4rs::{BoltList, BoltNode, BoltRelation, BoltType, Graph, Node, query, Relation};
use crate::graph_db::{EdgeData, GraphDbFunc, GraphSchema, NodeData, SPO};

pub struct Neo4j {
    graph: Graph,
    db_name: String,
}

impl Neo4j {

    fn parse_bolt_node_id_label(n: BoltNode) -> (String, String) {
        let id = n.id.value.to_string();
        let first_label = n.labels.value.get(0)
            .map(|label| label.to_string())
            .unwrap_or_else(|| "none".to_string());
        (id, first_label)
    }

    fn parse_bolt_node(n: BoltNode) -> Result<NodeData, Box<dyn Error>> {
        let mut node_data = NodeData::default();
        for (_, key) in n.properties.value.keys().into_iter().enumerate() {
            let i = n.get::<BoltType>(key.value.as_str()).expect("None property");
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
        let (id, label) = Self::parse_bolt_node_id_label(n);
        node_data.id = id;
        node_data.tag = label;
        Ok(node_data)
    }

    fn parse_bolt_edge(e: BoltRelation) -> Result<EdgeData, Box<dyn Error>> {
        let mut edge_data = EdgeData::default();
        edge_data.start_node_id = e.start_node_id.value.to_string();
        edge_data.end_node_id = e.end_node_id.value.to_string();
        edge_data.typ = e.typ.value;
        Ok(edge_data)
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
            let node: BoltNode = row.get("n").unwrap();
            match Self::parse_bolt_node(node) {
                Ok(n) => {
                    nodes.push(n);
                }
                Err(err) => {
                    eprintln!("Format node failed：{}", err)
                }
            }
        }
        nodes
    }

    async fn edges(&self) -> Vec<EdgeData> {
        let mut edges: Vec<EdgeData> = Vec::new();
        let mut result =
            self.graph.execute_on(self.db_name.as_str(), query("match ()-[e]->() return e")).await.unwrap();

        while let Ok(Some(row)) = result.next().await {
            let relation: BoltRelation = row.get("e").unwrap();
            match Self::parse_bolt_edge(relation) {
                Ok(e) => {
                    edges.push(e);
                }
                Err(err) => {
                    eprintln!("Format edge failed：{}", err)
                }
            }

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
                        let (id, label) = Self::parse_bolt_node_id_label(val);
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

    async fn subgraph(&self, id: &str) -> Result<(Vec<NodeData>, Vec<EdgeData>), Box<dyn Error>> {
        let cypher = "MATCH (n)<-[r]->(m) WHERE ID(n) = {id} RETURN r,m".replace("{id}", id);
        let mut result =
            self.graph.execute_on(self.db_name.as_str(), query(cypher.as_str())).await?;
        let mut node_list = vec![];
        let mut edge_list = vec![];

        while let Ok(Some(row)) = result.next().await {
            // n 自己的信息
            // n代号 r m 这里只需要r m
            if let Ok(m) = Self::parse_bolt_node(row.get("m")?) {
                node_list.push(m);
            }
            if let Ok(e) = Self::parse_bolt_edge(row.get("r")?) {
                edge_list.push(e);
            }
        }
        Ok((node_list, edge_list))
    }
}
