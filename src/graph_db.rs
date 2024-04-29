use std::collections::HashMap;
use std::error::Error;
use std::future::Future;

#[allow(unused)]
pub mod nebula_graph_db;
pub mod neo4j_graph_db;

#[derive(Default, Debug)]
pub struct NodeData {
    pub id: String,
    pub tag: String,
    pub properties: HashMap<String, String>
}

impl NodeData {

    // tag p1=v1 p2=v2 ...
    pub fn format(&self) -> String {
        let mut s = String::new();
        s.push_str(self.tag.as_str());
        s.push(' ');
        for (k, v) in self.properties.iter() {
            s.push_str(k);
            s.push('=');
            s.push_str(v);
            s.push(' ');
        }
        s
    }
}

#[derive(Default, Debug)]
pub struct EdgeData {
    pub start_node_id: String,
    pub end_node_id: String,
    pub typ: String,
}

impl EdgeData {

}

pub type SPO = (String, String, String);

#[derive(Debug)]
pub struct GraphSchema {
    spo_list: Vec<SPO>
}

impl GraphSchema {
    pub fn format(&self) -> String {
        let mut graph_schema_str = String::new();
        for (subject, predicate, object) in self.spo_list.iter() {
            let spo_str = format!("({}->{}->{})\n", subject, predicate, object);
            graph_schema_str.push_str(&spo_str);
        }
        graph_schema_str
    }
}

pub trait GraphDbFunc {

    type ConnParams;

    fn connect(params: Self::ConnParams) -> impl Future<Output = Self>;

    fn vertexes(&self) -> impl Future<Output = Vec<NodeData>>;

    fn edges(&self) -> impl Future<Output = Vec<EdgeData>>;

    // [tag1 - edge1 - tag2, ...]
    fn graph_schema(&self) -> impl Future<Output = GraphSchema>;

    fn subgraph(&self, id: &str) -> impl Future<Output = Result<(Vec<NodeData>, Vec<EdgeData>), Box<dyn Error>>>;

}