use fbthrift_transport::{AsyncTransport, AsyncTransportConfiguration};
use nebula_client::v3::{GraphClient, GraphQuery, GraphTransportResponseHandler};
use crate::graph_db::abs_graph_db::GraphDbFunc;
struct NebulaGraph {
    host: String
}

impl GraphDbFunc for NebulaGraph {
    async fn connect(&self) {
        let domain = "127.0.0.1";
        let port: u16 = 9669;
        let username = "root";
        let password = "nebula ";

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
    }

    fn vertexes() {

    }

    fn edges() {
        todo!()
    }
}
