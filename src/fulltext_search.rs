use std::future::Future;
use crate::fulltext_search::meilisearch::FulltextSearchEntity;

pub mod meilisearch;


pub trait FulltextSearch {

    const INDEXES_NAME: &'static str = "knowledge_graph_data";

    fn insert(&self, data: Vec<FulltextSearchEntity>);

    fn search(&self, input: &str) -> impl Future<Output = Vec<FulltextSearchEntity>>;

}