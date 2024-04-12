use std::collections::HashMap;
use std::future::Future;
use neo4rs::BoltType;

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

}

#[derive(Default, Debug)]
pub struct EdgeData {
    start_node_id: String,
    end_node_id: String,
    typ: String,
}

impl EdgeData {

}

pub trait GraphDbFunc {

    type ConnParams;

    fn connect(params: Self::ConnParams) -> impl std::future::Future<Output = Self> + Send where Self: Sized;

    fn vertexes(&self) -> impl Future<Output = Vec<NodeData>>;

    fn edges(&self) -> impl Future<Output = Vec<EdgeData>>;
}
// enum DatabaseType {
//     Neo4j,
//     Nebula,
// }


// fn create_database(db_type: DatabaseType, connection_string: &str) -> Box<dyn GraphDbFunc> {
//     match db_type {
//         DatabaseType::Neo4j => Box::new( NebulaGraph{
//             // connection_string: connection_string.to_string(),
//         }),
//         DatabaseType::Nebula => Box::new(GremlinDatabase {
//             connection_string: connection_string.to_string(),
//             // 初始化Gremlin特有参数
//         }),
//         // 处理其他数据库类型
//     }
// }

