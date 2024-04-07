#[allow(unused)]
pub mod nebula_graph_db;
pub mod neo4j_graph_db;


pub trait GraphDbFunc {

    type ConnParams;

    fn connect(params: Self::ConnParams) -> impl std::future::Future<Output = Self> + Send where Self: Sized;

    fn vertexes(&self) -> impl std::future::Future<Output = ()>;

    fn edges(&self) {}

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

