use graph2vector::graph_db::GraphDbFunc;
use graph2vector::graph_db::nebula_graph_db::{NebulaGraph, NebulaGraphParams};

fn main() {
    println!("ok");
    // 先获取调用参数，检查参数
    let obj = NebulaGraph::connect(
        NebulaGraphParams{host: "127.0.0.1", port: 9661, user: "root", pwd: "nebula", db_name: "test"});
    // 实例化三个服务、确认无报错

    // 依次调用三个服务、返回成功
}
