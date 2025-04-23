use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wp_term_taxonomy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub term_taxonomy_id: u64,
    pub term_id: u64,
    pub taxonomy: String,
    pub description: String,
    pub parent: u64,
    pub count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::term::Entity",
        from = "Column::TermId",
        to = "super::term::Column::TermId"
    )]
    Term,
    #[sea_orm(has_many = "super::term_relationship::Entity")]
    TermRelationships,
}

impl Related<super::term::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Term.def()
    }
}

impl Related<super::term_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TermRelationships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_categories(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<(Model, super::term::Model)>, u64), DbErr> {
        let taxonomies = Self::find()
            .filter(Column::Taxonomy.eq("category"))
            .all(db)
            .await?;

        let mut results = Vec::new();
        for taxonomy in taxonomies {
            if let Some(term) = super::term::Entity::find_by_id(taxonomy.term_id)
                .one(db)
                .await?
            {
                results.push((taxonomy, term));
            }
        }

        let total = results.len() as u64;
        let start = (page - 1) * page_size;
        let end = std::cmp::min(start + page_size, total);

        if start < total {
            let paginated_results = results
                .into_iter()
                .skip(start as usize)
                .take((end - start) as usize)
                .collect();
            Ok((paginated_results, total))
        } else {
            Ok((Vec::new(), total))
        }
    }
}
