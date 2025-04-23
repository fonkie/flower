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
    // Additional validation logic 
    if page_size > 100 {
        return Err(ApiError::BadRequest("Page size exceeds maximum limit of 100".to_string()));
    }

    if page == 0 {
        return Err(ApiError::BadRequest("Page number must be greater than zero".to_string()));
    }

    if let Some(search_term) = &search {
        if search_term.len() < 3 {
            return Err(ApiError::BadRequest(
                "Search term must be at least 3 characters long".to_string(),
            ));
        }
    }

    // Try to retrieve posts from the database
    let (posts, total) = match post::Entity::find_posts(
        db,
        post_type.clone(),
        post_status.clone(),
        page,
        page_size,
        search.clone(),
        author_id,
    )
    .await
    {
        Ok(result) => result,
        Err(err) => {
            // Enhanced error handling
            let error_msg = format!(
                "Failed to retrieve posts with filters - type: {:?}, status: {:?}, page: {}, page_size: {}, search: {:?}, author: {:?}. Error: {}",
                post_type, post_status, page, page_size, search, author_id, err
            );
            return Err(ApiError::InternalServerError(error_msg));
        }
    };

    Ok((posts, total))
}

pub async fn get_post_by_id(
    db: &DatabaseConnection,
    post_id: i32,
    ensure_published: bool,
) -> Result<post::Model, ApiError> {
    if post_id <= 0 {
        return Err(ApiError::BadRequest("Post ID must be a positive integer".to_string()));
    }

    let post = match post::Entity::find_by_id(db, post_id).await {
        Ok(result) => result,
        Err(err) => {
            return Err(ApiError::InternalServerError(format!(
                "Failed to retrieve post with ID {}: {}",
                post_id, err
            )));
        }
    };

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
    // Validate input parameters
    if post_type.is_empty() {
        return Err(ApiError::BadRequest("Post type cannot be empty".to_string()));
    }

    if page == 0 {
        return Err(ApiError::BadRequest("Page number must be greater than zero".to_string()));
    }

    if page_size == 0 {
        return Err(ApiError::BadRequest("Page size must be greater than zero".to_string()));
    }

    if page_size > 100 {
        return Err(ApiError::BadRequest("Page size exceeds maximum limit of 100".to_string()));
    }

    // Try to retrieve posts by type
    let (posts, total) = match post::Entity::find_by_type(db, post_type, post_status, page, page_size).await {
        Ok(result) => result,
        Err(err) => {
            return Err(ApiError::InternalServerError(format!(
                "Failed to retrieve posts of type '{}': {}",
                post_type, err
            )));
        }
    };

    Ok((posts, total))
}

pub async fn get_post_types(db: &DatabaseConnection) -> Result<Vec<(String, i64, i64)>, ApiError> {
    match post::Entity::get_post_types(db).await {
        Ok(post_types) => Ok(post_types),
        Err(err) => {
            Err(ApiError::InternalServerError(format!(
                "Failed to retrieve post types: {}",
                err
            )))
        }
    }
}

pub async fn get_posts_by_category(
    db: &DatabaseConnection,
    category_id: i32,
    page: u64,
    page_size: u64,
) -> Result<(Vec<post::Model>, u64), ApiError> {
    // Validate input parameters
    if category_id <= 0 {
        return Err(ApiError::BadRequest("Category ID must be a positive integer".to_string()));
    }

    if page == 0 {
        return Err(ApiError::BadRequest("Page number must be greater than zero".to_string()));
    }

    if page_size == 0 {
        return Err(ApiError::BadRequest("Page size must be greater than zero".to_string()));
    }

    if page_size > 100 {
        return Err(ApiError::BadRequest("Page size exceeds maximum limit of 100".to_string()));
    }

    // Try to retrieve posts by category
    match post::Entity::find_by_category(db, category_id, page, page_size).await {
        Ok(result) => Ok(result),
        Err(err) => {
            Err(ApiError::InternalServerError(format!(
                "Failed to retrieve posts for category ID {}: {}",
                category_id, err
            )))
        }
    }
}

pub async fn get_post_meta(
    db: &DatabaseConnection,
    post_id: i32,
) -> Result<HashMap<String, String>, ApiError> {
    // Validate post_id
    if post_id <= 0 {
        return Err(ApiError::BadRequest("Post ID must be a positive integer".to_string()));
    }

    // First check if the post exists
    let post = get_post_by_id(db, post_id, true).await?;

    // Try to retrieve post metadata
    match postmeta::Entity::find_metadata_map(db, post.id).await {
        Ok(meta) => Ok(meta),
        Err(err) => {
            Err(ApiError::InternalServerError(format!(
                "Failed to retrieve metadata for post ID {}: {}",
                post_id, err
            )))
        }
    }
}

pub async fn get_categories(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(Vec<(term_taxonomy::Model, term::Model)>, u64), ApiError> {
    // Validate input parameters
    if page == 0 {
        return Err(ApiError::BadRequest("Page number must be greater than zero".to_string()));
    }

    if page_size == 0 {
        return Err(ApiError::BadRequest("Page size must be greater than zero".to_string()));
    }

    if page_size > 100 {
        return Err(ApiError::BadRequest("Page size exceeds maximum limit of 100".to_string()));
    }

    // Try to retrieve categories
    match term_taxonomy::Entity::find_categories(db, page, page_size).await {
        Ok(categories) => Ok(categories),
        Err(err) => {
            Err(ApiError::InternalServerError(format!(
                "Failed to retrieve categories: {}",
                err
            )))
        }
    }
}