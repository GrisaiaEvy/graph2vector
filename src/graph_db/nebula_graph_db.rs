use std::ffi::c_int;
use fbthrift_transport::{AsyncTransport, AsyncTransportConfiguration};
use nebula_client::v1::GraphSession;
use nebula_client::v3::{GraphClient, GraphQuery, GraphTransportResponseHandler};
use crate::graph_db::GraphDbFunc;

pub struct NebulaGraph {

}

pub struct NebulaGraphParams {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pwd: String,
    pub db_name: String,
}
// 从文件读取，或者仔细研究下文档
impl GraphDbFunc for NebulaGraph {

    type ConnParams = NebulaGraphParams;

    async fn connect(params: Self::ConnParams) -> Self {
        let domain = params.host;
        let port: u16 = params.port;
        let username = params.user;
        let password = params.pwd;

        println!("v3_graph_client {domain} {port} {username} {password}",);

        //
        let addr = format!("{domain}:{port}");

        //
        let transport = AsyncTransport::with_tokio_tcp_connect(
            addr,
            AsyncTransportConfiguration::new(GraphTransportResponseHandler),
        )
            .await?;
        let client = GraphClient::new(transport);

        let mut session = client
            .authenticate(&username.as_bytes().to_vec(), &password.as_bytes().to_vec())
            .await?;
        let res = session.show_hosts().await?;
        NebulaGraph {}
    }

    fn vertexes(&self) {

    }

    fn edges(&self) {
        todo!()
    }
}
