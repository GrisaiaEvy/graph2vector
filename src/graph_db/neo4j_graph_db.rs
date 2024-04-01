use neo4rs::Graph;
use crate::graph_db::GraphDbFunc;

struct Neo4j {

}

struct Neo4jParams {
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

        Neo4j {}
    }
}
