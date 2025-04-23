use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::NullOrdering;
use sea_orm::{Condition, EntityOrSelect, Order, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wp_posts")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "ID")]
    pub id: u64,
    pub post_author: u64,

    pub post_date: Option<NaiveDateTime>,
    pub post_date_gmt: Option<NaiveDateTime>,
    pub post_content: String,
    pub post_title: String,
    pub post_excerpt: String,
    pub post_status: String,
    pub comment_status: String,
    pub ping_status: String,
    pub post_password: String,
    pub post_name: String,
    pub to_ping: String,
    pub pinged: String,

    pub post_modified: Option<NaiveDateTime>,
    pub post_modified_gmt: Option<NaiveDateTime>,
    pub post_content_filtered: String,
    pub post_parent: u64,
    pub guid: String,
    pub menu_order: i32,
    pub post_type: String,
    pub post_mime_type: String,
    pub comment_count: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::postmeta::Entity")]
    Postmeta,
    #[sea_orm(has_many = "super::term_relationship::Entity")]
    TermRelationships,
}

impl Related<super::postmeta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Postmeta.def()
    }
}

impl Related<super::term_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TermRelationships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_by_id(db: &DatabaseConnection, id: u64) -> Result<Option<Model>, DbErr> {
        Entity::find().filter(Column::Id.eq(id)).one(db).await
    }

    pub async fn find_posts(
        db: &DatabaseConnection,
        post_type: Option<String>,
        post_status: Option<String>,
        page: u64,
        page_size: u64,
        search: Option<String>,
        author_id: Option<u64>,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut query = Self::find();

        if let Some(post_type) = post_type {
            query = query.filter(Column::PostType.eq(post_type));
        }

        if let Some(post_status) = post_status {
            query = query.filter(Column::PostStatus.eq(post_status));
        }

        if let Some(author_id) = author_id {
            query = query.filter(Column::PostAuthor.eq(author_id));
        }

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(Column::PostTitle.contains(&search))
                    .add(Column::PostContent.contains(&search)),
            );
        }

        let total = query.clone().count(db).await?;

        let posts = query
            .order_by_with_nulls(Column::PostDate, Order::Desc, NullOrdering::Last)
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await?;

        Ok((posts, total))
    }

    pub async fn find_by_type(
        db: &DatabaseConnection,
        post_type: &str,
        post_status: Option<&str>,
        page: u64,
        page_size: u64,
        search: Option<String>,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut query = Self::find().filter(Column::PostType.eq(post_type));

        if let Some(status) = post_status {
            query = query.filter(Column::PostStatus.eq(status));
        }

        if let Some(search_term) = search {
            query = query.filter(
                Condition::any()
                    .add(Column::PostTitle.contains(&search_term))
                    .add(Column::PostContent.contains(&search_term)),
            );
        }

        let total = query.clone().count(db).await?;

        let posts = query
            .order_by_with_nulls(Column::PostDate, Order::Desc, NullOrdering::Last)
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await?;

        Ok((posts, total))
    }

    pub async fn get_post_types(db: &DatabaseConnection) -> Result<Vec<(String, i64, i64)>, DbErr> {
        let post_types = Self::find()
            .select_only()
            .column(Column::PostType)
            .distinct()
            .order_by(Column::PostType, Order::Asc)
            .into_tuple::<String>()
            .all(db)
            .await?;

        let mut result = Vec::new();

        for post_type in post_types {
            let total_count = Self::find()
                .filter(Column::PostType.eq(&post_type))
                .count(db)
                .await?;

            let published_count = Self::find()
                .filter(Column::PostType.eq(&post_type))
                .filter(Column::PostStatus.eq("publish"))
                .count(db)
                .await?;

            result.push((post_type, total_count as i64, published_count as i64));
        }

        result.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(result)
    }

    pub async fn find_by_category(
        db: &DatabaseConnection,
        category_id: i32,
        page: u64,
        page_size: u64,
        search: Option<String>,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let term_taxonomy_id = super::term_taxonomy::Entity::find()
            .filter(super::term_taxonomy::Column::TermId.eq(category_id))
            .filter(super::term_taxonomy::Column::Taxonomy.eq("category"))
            .select()
            .column(super::term_taxonomy::Column::TermTaxonomyId)
            .into_tuple::<u64>()
            .one(db)
            .await?;

        if let Some(term_taxonomy_id) = term_taxonomy_id {
            let post_ids = super::term_relationship::Entity::find()
                .filter(super::term_relationship::Column::TermTaxonomyId.eq(term_taxonomy_id))
                .select()
                .column(super::term_relationship::Column::ObjectId)
                .into_tuple::<u64>()
                .all(db)
                .await?;

            if post_ids.is_empty() {
                return Ok((Vec::new(), 0));
            }

            let mut query = Self::find()
                .filter(Column::Id.is_in(post_ids))
                .filter(Column::PostStatus.eq("publish"));

            if let Some(search_term) = search {
                query = query.filter(
                    Condition::any()
                        .add(Column::PostTitle.contains(&search_term))
                        .add(Column::PostContent.contains(&search_term)),
                );
            }

            let total = query.clone().count(db).await?;

            let posts = query
                .order_by_with_nulls(Column::PostDate, Order::Desc, NullOrdering::Last)
                .paginate(db, page_size)
                .fetch_page(page - 1)
                .await?;

            Ok((posts, total))
        } else {
            Ok((Vec::new(), 0))
        }
    }
}
