pub trait GraphDbFunc {

    fn connect(&self) {}

    fn vertexes() {}

    fn edges() {}

}
enum DatabaseType {
    Neo4j,
    Nebula,
}
//
// fn create_database(db_type: DatabaseType, connection_string: &str) -> Box<dyn GraphDatabase> {
//     match db_type {
//         DatabaseType::Neo4j => Box::new(Neo4jDatabase {
//             connection_string: connection_string.to_string(),
//         }),
//         DatabaseType::Gremlin => Box::new(GremlinDatabase {
//             connection_string: connection_string.to_string(),
//             // 初始化Gremlin特有参数
//         }),
//         // 处理其他数据库类型
//     }
// }

