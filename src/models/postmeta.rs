use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wp_postmeta")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub meta_id: i32,
    pub post_id: i32,
    pub meta_key: String,
    pub meta_value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Additional query methods
impl Entity {
    // Get post metadata by post ID
    pub async fn find_by_post_id(
        db: &DatabaseConnection,
        post_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(Column::PostId.eq(post_id))
            .all(db)
            .await
    }
    
    // Get metadata as a key-value map
    pub async fn find_metadata_map(
        db: &DatabaseConnection,
        post_id: i32,
    ) -> Result<std::collections::HashMap<String, String>, DbErr> {
        let metadata = Self::find_by_post_id(db, post_id).await?;
        let map = metadata
            .into_iter()
            .map(|meta| (meta.meta_key, meta.meta_value))
            .collect();
        
        Ok(map)
    }
}