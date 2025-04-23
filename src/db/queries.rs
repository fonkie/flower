use crate::error::ApiError;
use crate::models::{post, postmeta, term, term_taxonomy};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

pub async fn get_posts(
    db: &DatabaseConnection,
    post_type: Option<String>,
    post_status: Option<String>,
    page: u64,
    page_size: u64,
    search: Option<String>,
    author_id: Option<i32>,
) -> Result<(Vec<post::Model>, u64), ApiError> {
    let (posts, total) = post::Entity::find_posts(
        db,
        post_type,
        post_status,
        page,
        page_size,
        search,
        author_id,
    )
    .await
    .map_err(ApiError::from)?;

    Ok((posts, total))
}

pub async fn get_post_by_id(
    db: &DatabaseConnection,
    post_id: i32,
    ensure_published: bool,
) -> Result<post::Model, ApiError> {
    let post = post::Entity::find_by_id(db, post_id)
        .await
        .map_err(ApiError::from)?;

    match post {
        Some(post) => {
            if ensure_published && post.post_status != "publish" {
                Err(ApiError::NotFound(format!(
                    "Published post with ID {} not found",
                    post_id
                )))
            } else {
                Ok(post)
            }
        }
        None => Err(ApiError::NotFound(format!(
            "Post with ID {} not found",
            post_id
        ))),
    }
}

pub async fn get_posts_by_type(
    db: &DatabaseConnection,
    post_type: &str,
    post_status: Option<&str>,
    page: u64,
    page_size: u64,
) -> Result<(Vec<post::Model>, u64), ApiError> {
    let (posts, total) = post::Entity::find_by_type(db, post_type, post_status, page, page_size)
        .await
        .map_err(ApiError::from)?;

    Ok((posts, total))
}

pub async fn get_post_types(db: &DatabaseConnection) -> Result<Vec<(String, i64, i64)>, ApiError> {
    let post_types = post::Entity::get_post_types(db)
        .await
        .map_err(ApiError::from)?;

    Ok(post_types)
}

pub async fn get_posts_by_category(
    db: &DatabaseConnection,
    category_id: i32,
    page: u64,
    page_size: u64,
) -> Result<(Vec<post::Model>, u64), ApiError> {
    let (posts, total) = post::Entity::find_by_category(db, category_id, page, page_size)
        .await
        .map_err(ApiError::from)?;

    Ok((posts, total))
}

pub async fn get_post_meta(
    db: &DatabaseConnection,
    post_id: i32,
) -> Result<HashMap<String, String>, ApiError> {
    let post = get_post_by_id(db, post_id, true).await?;

    let meta = postmeta::Entity::find_metadata_map(db, post.id)
        .await
        .map_err(ApiError::from)?;

    Ok(meta)
}

pub async fn get_categories(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(Vec<(term_taxonomy::Model, term::Model)>, u64), ApiError> {
    let (categories, total) = term_taxonomy::Entity::find_categories(db, page, page_size)
        .await
        .map_err(ApiError::from)?;

    Ok((categories, total))
}
