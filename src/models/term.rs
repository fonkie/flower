use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wp_terms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub term_id: i32,
    pub name: String,
    pub slug: String,
    pub term_group: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::term_taxonomy::Entity")]
    TermTaxonomies,
}

impl Related<super::term_taxonomy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TermTaxonomies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}