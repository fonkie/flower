use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wp_term_relationships")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "object_id")]
    pub object_id: u64,
    #[sea_orm(primary_key)]
    pub term_taxonomy_id: i32,
    pub term_order: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::ObjectId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::term_taxonomy::Entity",
        from = "Column::TermTaxonomyId",
        to = "super::term_taxonomy::Column::TermTaxonomyId"
    )]
    TermTaxonomy,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::term_taxonomy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TermTaxonomy.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
