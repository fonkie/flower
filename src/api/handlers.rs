use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::api::responses::{Category, PaginatedResponse, Post, PostMeta, PostType, RootResponse};
use crate::db::queries;
use crate::error::ApiError;

pub async fn root() -> impl Responder {
    let response = RootResponse {
        title: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Read-only RESTful API for WordPress data".to_string(),
    };
    
    HttpResponse::Ok().json(response)
}
pub async fn get_posts(
    query: web::Query<GetPostsQuery>,
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    // Validate page and page_size parameters
    if let Some(page) = query.page {
        if page == 0 {
            return Err(ApiError::BadRequest("Page number must be greater than 0".to_string()));
        }
    }

    if let Some(page_size) = query.page_size {
        if page_size == 0 {
            return Err(ApiError::BadRequest("Page size must be greater than 0".to_string()));
        }
        if page_size > 100 {
            return Err(ApiError::BadRequest("Maximum page size is 100".to_string()));
        }
    }

    // Validate post_type if provided
    if let Some(post_type) = &query.post_type {
        if post_type.is_empty() {
            return Err(ApiError::BadRequest("Post type cannot be empty".to_string()));
        }
    }

    let post_status = if let Some(status) = &query.post_status {
        // Validate post_status
        let valid_statuses = ["publish", "draft", "private", "pending", "future", "trash", "auto-draft"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(ApiError::BadRequest(format!(
                "Invalid post status: {}. Valid statuses are: {}",
                status,
                valid_statuses.join(", ")
            )));
        }
        Some(status.to_string())
    } else {
        Some("publish".to_string())
    };

    let post_type = query.post_type.clone();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10).min(100);
    let search = query.search.clone();
    let author_id = query.author_id;

    let (posts, total) = queries::get_posts(
        &db,
        post_type,
        post_status,
        page,
        page_size,
        search,
        author_id,
    )
    .await?;

    let post_responses: Vec<Post> = posts.into_iter().map(Post::from).collect();

    let response = PaginatedResponse::new(post_responses, total, page, page_size);

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_post(
    path: web::Path<i32>,
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    let post_id = path.into_inner();
    
    // Validate post_id
    if post_id <= 0 {
        return Err(ApiError::BadRequest("Post ID must be a positive integer".to_string()));
    }

    let post = queries::get_post_by_id(&db, post_id, true).await?;

    let post_response = Post::from(post);

    Ok(HttpResponse::Ok().json(post_response))
}

pub async fn get_post_meta(
    path: web::Path<i32>,
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    let post_id = path.into_inner();
    
    // Validate post_id
    if post_id <= 0 {
        return Err(ApiError::BadRequest("Post ID must be a positive integer".to_string()));
    }

    let meta = queries::get_post_meta(&db, post_id).await?;

    // Check if we got empty metadata
    if meta.is_empty() {
        // Note: We don't consider empty metadata an error, but we could if the business logic required it
        // return Err(ApiError::NotFound(format!("No metadata found for post {}", post_id)));
    }

    let response = PostMeta { meta };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_post_types(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    let post_types = queries::get_post_types(&db).await?;

    let response: Vec<PostType> = post_types
        .into_iter()
        .map(|(name, count, published_count)| PostType {
            name,
            count,
            published_count,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_posts_by_type(
    path: web::Path<String>,
    query: web::Query<GetPostsTypeQuery>,
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    let post_type = path.into_inner();
    
    // Validate post_type
    if post_type.is_empty() {
        return Err(ApiError::BadRequest("Post type cannot be empty".to_string()));
    }

    // Validate pagination parameters
    if let Some(page) = query.page {
        if page == 0 {
            return Err(ApiError::BadRequest("Page number must be greater than 0".to_string()));
        }
    }

    if let Some(page_size) = query.page_size {
        if page_size == 0 {
            return Err(ApiError::BadRequest("Page size must be greater than 0".to_string()));
        }
        if page_size > 100 {
            return Err(ApiError::BadRequest("Maximum page size is 100".to_string()));
        }
    }

    // Validate post_status if provided
    let post_status = if let Some(status) = &query.post_status {
        let valid_statuses = ["publish", "draft", "private", "pending", "future", "trash", "auto-draft"];
        if !valid_statuses.contains(&status.as_str()) {
            return Err(ApiError::BadRequest(format!(
                "Invalid post status: {}. Valid statuses are: {}",
                status,
                valid_statuses.join(", ")
            )));
        }
        Some(status.as_str())
    } else {
        None
    };

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10).min(100);

    let (posts, total) =
        queries::get_posts_by_type(&db, &post_type, post_status, page, page_size).await?;

    let post_responses: Vec<Post> = posts.into_iter().map(Post::from).collect();

    let response = PaginatedResponse::new(post_responses, total, page, page_size);

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_categories(
    query: web::Query<GetCategoriesQuery>,
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    // Validate pagination parameters
    if let Some(page) = query.page {
        if page == 0 {
            return Err(ApiError::BadRequest("Page number must be greater than 0".to_string()));
        }
    }

    if let Some(page_size) = query.page_size {
        if page_size == 0 {
            return Err(ApiError::BadRequest("Page size must be greater than 0".to_string()));
        }
        if page_size > 100 {
            return Err(ApiError::BadRequest("Maximum page size is 100".to_string()));
        }
    }

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);

    let (categories, total) = queries::get_categories(&db, page, page_size).await?;

    let category_responses: Vec<Category> = categories.into_iter().map(Category::from).collect();

    let response = PaginatedResponse::new(category_responses, total, page, page_size);

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_posts_by_category(
    path: web::Path<i32>,
    query: web::Query<GetPostsCategoryQuery>,
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, ApiError> {
    let category_id = path.into_inner();
    
    // Validate category_id
    if category_id <= 0 {
        return Err(ApiError::BadRequest("Category ID must be a positive integer".to_string()));
    }

    // Validate pagination parameters
    if let Some(page) = query.page {
        if page == 0 {
            return Err(ApiError::BadRequest("Page number must be greater than 0".to_string()));
        }
    }

    if let Some(page_size) = query.page_size {
        if page_size == 0 {
            return Err(ApiError::BadRequest("Page size must be greater than 0".to_string()));
        }
        if page_size > 100 {
            return Err(ApiError::BadRequest("Maximum page size is 100".to_string()));
        }
    }

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10).min(100);

    let (posts, total) = queries::get_posts_by_category(&db, category_id, page, page_size).await?;

    let post_responses: Vec<Post> = posts.into_iter().map(Post::from).collect();

    let response = PaginatedResponse::new(post_responses, total, page, page_size);

    Ok(HttpResponse::Ok().json(response))
}

#[derive(serde::Deserialize)]
pub struct GetPostsQuery {
    pub post_type: Option<String>,
    pub post_status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    pub author_id: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct GetPostsTypeQuery {
    pub post_status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct GetCategoriesQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(serde::Deserialize)]
pub struct GetPostsCategoryQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}