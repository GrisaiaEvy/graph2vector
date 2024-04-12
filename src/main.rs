use fastembed::TextEmbedding;
use graph2vector::embedding_strategy::StrategyBuilder;
use graph2vector::graph_db::GraphDbFunc;
use graph2vector::graph_db::neo4j_graph_db::{Neo4j, Neo4jParams};

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new(/* ... */) -> FooBuilder {
        FooBuilder {
            bar: String::from("X")
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn builder(self) -> Foo {
        Foo {
            bar: self.bar
        }
    }
}

#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y")
    };
    let foo_builder = FooBuilder::new().name(String::from("Y")).builder();
    assert_eq!(foo, foo_builder)
}


#[tokio::main]
async fn main() {
    // println!("整体开始执行");
    // let neo4j = Neo4j::connect(Neo4jParams{host: String::from("127.0.0.1"),
    //     port: 7687, user: String::from("neo4j"), pwd: String::from("123456"), db_name: String::from("politics")});
    // println!("获取图数据库实例");
    // let nodes = neo4j.await.vertexes().await;
    //
    // let edges = neo4j.await.edges().await;
    // let &x = edges.get(0).unwrap();
    // let y = &x;
    // 创建策略 建造者模式
    // Strategy.xxxStrategy().graph().vec().vecDb();
    let strategy = StrategyBuilder::new().graph().vectorize().vector_db().build_entity_strategy();

}
