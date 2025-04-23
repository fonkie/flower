use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{Condition, QueryOrder, QuerySelect, Order, EntityOrSelect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wp_posts")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "ID")]
    pub id: i32,
    pub post_author: i32,
    pub post_date: NaiveDateTime,
    pub post_date_gmt: NaiveDateTime,
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
    pub post_modified: NaiveDateTime,
    pub post_modified_gmt: NaiveDateTime,
    pub post_content_filtered: String,
    pub post_parent: i32,
    pub guid: String,
    pub menu_order: i32,
    pub post_type: String,
    pub post_mime_type: String,
    pub comment_count: i32,
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

// Additional query methods
impl Entity {
    // Get post by ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find().filter(Column::Id.eq(id)).one(db).await
    }
    
    // Get posts with pagination and filtering
    pub async fn find_posts(
        db: &DatabaseConnection,
        post_type: Option<String>,
        post_status: Option<String>,
        page: u64,
        page_size: u64,
        search: Option<String>,
        author_id: Option<i32>,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut query = Self::find();
        
        // Apply filters
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
        
        // Get total count
        let total = query.clone().count(db).await?;
        
        // Add pagination and ordering
        let posts = query
            .order_by(Column::PostDate, Order::Desc)
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await?;
        
        Ok((posts, total))
    }
    
    // Get posts by type
    pub async fn find_by_type(
        db: &DatabaseConnection,
        post_type: &str,
        post_status: Option<&str>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut query = Self::find().filter(Column::PostType.eq(post_type));
        
        if let Some(status) = post_status {
            query = query.filter(Column::PostStatus.eq(status));
        }
        
        // Get total count
        let total = query.clone().count(db).await?;
        
        // Add pagination and ordering
        let posts = query
            .order_by(Column::PostDate, Order::Desc)
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await?;
        
        Ok((posts, total))
    }
    
    // Get post types with counts
    pub async fn get_post_types(db: &DatabaseConnection) -> Result<Vec<(String, i64, i64)>, DbErr> {
        // This query will:
        // 1. Group posts by post_type
        // 2. Count total posts
        // 3. Count published posts
        let result = Self::find()
            .select()
            .column(Column::PostType)
            .column_as(Expr::count(Expr::col(Column::Id)), "count")
            .column_as(
                Expr::cust_with_expr("SUM(CASE WHEN post_status = 'publish' THEN 1 ELSE 0 END)", Expr::value("")),
                "published_count",
            )
            .group_by(Column::PostType)
            .order_by(Expr::cust("count"), Order::Desc)
            .into_tuple::<(String, i64, i64)>()
            .all(db)
            .await?;
        
        Ok(result)
    }
    
    // Get posts by category
    pub async fn find_by_category(
        db: &DatabaseConnection,
        category_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        // This requires a join across multiple tables:
        // 1. wp_posts
        // 2. wp_term_relationships
        // 3. wp_term_taxonomy

        // First, get the term_taxonomy_id for the category
        let term_taxonomy_id = super::term_taxonomy::Entity::find()
            .filter(super::term_taxonomy::Column::TermId.eq(category_id))
            .filter(super::term_taxonomy::Column::Taxonomy.eq("category"))
            .select()
            .column(super::term_taxonomy::Column::TermTaxonomyId)
            .into_tuple::<i32>()
            .one(db)
            .await?;
        
        if let Some(term_taxonomy_id) = term_taxonomy_id {
            // Get post ids that belong to this category
            let post_ids = super::term_relationship::Entity::find()
                .filter(super::term_relationship::Column::TermTaxonomyId.eq(term_taxonomy_id))
                .select()
                .column(super::term_relationship::Column::ObjectId)
                .into_tuple::<i32>()
                .all(db)
                .await?;
            
            if post_ids.is_empty() {
                return Ok((Vec::new(), 0));
            }
            
            // Now get the posts with these IDs
            let query = Self::find()
                .filter(Column::Id.is_in(post_ids))
                .filter(Column::PostStatus.eq("publish"));
            
            // Get total count
            let total = query.clone().count(db).await?;
            
            // Add pagination and ordering
            let posts = query
                .order_by(Column::PostDate, Order::Desc)
                .paginate(db, page_size)
                .fetch_page(page - 1)
                .await?;
            
            Ok((posts, total))
        } else {
            // Category not found
            Ok((Vec::new(), 0))
        }
    }
}