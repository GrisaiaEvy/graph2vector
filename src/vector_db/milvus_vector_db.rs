use fastembed::Embedding;
use milvus::client::Client;
use milvus::collection::Collection;
use milvus::data::FieldColumn;
use milvus::error::Error;
use milvus::schema::{CollectionSchema, CollectionSchemaBuilder, FieldSchema};
use crate::vector_db::VectorDbFunc;

pub struct Milvus {
    url: &str,
    client: Client,
    schema: CollectionSchema,
    collection: Collection
}

impl Milvus {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let schema = CollectionSchemaBuilder::new(Self::SCHEMA_NAME, "")
            .add_field(FieldSchema::new_primary_varchar(
                Self::ID_FIELD,
                "",
                false,
                1000
            ))
            .add_field(FieldSchema::new_float_vector(
                Self::VECTOR_FIELD,
                "",
                1024
            ))
            .add_field(FieldSchema::new_varchar(
                Self::CONTENT_FIELD,
                "",
                65535
            ))
            .add_field(FieldSchema::new_varchar(
                Self::METADATA_FIELD,
                "",
                65535
            ))
            .build()?;

        let client = Client::new(url).await?;
        let exists = client.has_collection(Self::SCHEMA_NAME).await?;

        let collection: Collection;
        if !exists {
            collection = client.create_collection(schema.clone(), None).await?;
        } else {
            collection = client.get_collection(Self::SCHEMA_NAME).await?;
        }
        collection.load_partition_list().await?;

        Ok(Milvus {
            url,
            client,
            schema,
            collection
        })
    }
}

impl VectorDbFunc for Milvus {
    async fn insert(&self, id: String, content: String, vector: Vec<f32>, metadata: String) {
        let data = vec![
            FieldColumn::new(Self.schema.get_field(Self::ID_FIELD)?, id.as_str()),
            FieldColumn::new(Self.schema.get_field(Self::VECTOR_FIELD)?, vector),
            FieldColumn::new(Self.schema.get_field(Self::CONTENT_FIELD)?, content.as_str()),
            FieldColumn::new(Self.schema.get_field(Self::METADATA_FIELD)?, metadata.as_str()),
        ];
        let r = self.collection.insert(data, None).await?;
        println!("{:?}", r)
    }

    async fn search(&self) {

    }
}