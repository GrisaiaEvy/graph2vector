use meilisearch_sdk::client::*;
use meilisearch_sdk::Settings;
use serde::{Deserialize, Serialize};
use crate::fulltext_search::FulltextSearch;

#[derive(Debug, Serialize, Deserialize)]
pub struct FulltextSearchEntity {
    id: String,
    content: String,
    metadata: String
}

pub struct MeiliSearch {
    client: Client,
    url: String,
    search_api_key: String
}

impl MeiliSearch {
    pub async fn new(url: String, search_api_key: String) -> MeiliSearch {
        let client = Client::new(&url, Some(&search_api_key));

        let settings = Settings::new()
            .with_ranking_rules(Self::ranking_rules());

        client.index(Self::INDEXES_NAME)
            .set_settings(&settings)
            .await
            .unwrap()
            .wait_for_completion(&client, None, None)
            .await
            .expect("Init meilisearch failed");

        MeiliSearch {
            client,
            url,
            search_api_key
        }
    }

    fn ranking_rules() -> [&'static str; 4] {
        ["word", "typos", "exactness", "proximity"]
    }
}

impl FulltextSearch for MeiliSearch {
    async fn insert(&self, data: Vec<FulltextSearchEntity>) {
        let result = self.client.index(Self::INDEXES_NAME)
            .add_or_update(&data, Some("id"))
            .await
            .unwrap()
            .wait_for_completion(&self.client, None, None)
            .await
            .unwrap();
        if result.is_failure() {
            panic!(
                "Encountered an error while sending the documents: {:?}",
                result.unwrap_failure()
            );
        }
    }


    async fn search(&self, input: &str) -> Vec<FulltextSearchEntity> {
        let vec = self.client.index("")
            .search()
            .with_query("")
            .execute::<FulltextSearchEntity>()
            .await
            .unwrap()
            .hits;
        println!("{:?}", vec);
        Vec::new()
    }
}


