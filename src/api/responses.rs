use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

use crate::models::{post, term, term_taxonomy};

#[derive(Serialize)]
pub struct RootResponse {
    pub title: String,
    pub version: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct Post {
    pub id: u64,
    pub post_title: String,
    pub post_content: String,
    pub post_excerpt: String,
    pub post_status: String,
    pub post_type: String,
    pub post_author: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub post_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub post_modified: DateTime<Utc>,
    pub guid: String,
    pub post_name: String,
    pub comment_count: i64,
}

impl From<post::Model> for Post {
    fn from(model: post::Model) -> Self {
        Post {
            id: model.id,
            post_title: model.post_title,
            post_content: model.post_content,
            post_excerpt: model.post_excerpt,
            post_status: model.post_status,
            post_type: model.post_type,
            post_author: model.post_author,
            post_date: DateTime::<Utc>::from_naive_utc_and_offset(
                model.post_date.unwrap(),
                Utc,
            ),
            post_modified: DateTime::<Utc>::from_naive_utc_and_offset(
                model.post_modified.unwrap(),
                Utc,
            ),
            guid: model.guid,
            post_name: model.post_name,
            comment_count: model.comment_count,
        }
    }
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub size: u64,
    pub pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, page: u64, size: u64) -> Self {
        let pages = if size > 0 {
            (total + size - 1) / size
        } else {
            0
        };

        PaginatedResponse {
            items,
            total,
            page,
            size,
            pages,
            has_next: page < pages,
            has_prev: page > 1,
        }
    }
}

#[derive(Serialize)]
pub struct PostType {
    pub name: String,
    pub count: i64,
    pub published_count: i64,
}

#[derive(Serialize)]
pub struct PostMeta {
    pub meta: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct Category {
    pub term_id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub parent: i32,
    pub count: i32,
}

impl From<(term_taxonomy::Model, term::Model)> for Category {
    fn from(data: (term_taxonomy::Model, term::Model)) -> Self {
        let (taxonomy, term) = data;

        Category {
            term_id: term.term_id,
            name: term.name,
            slug: term.slug,
            description: taxonomy.description,
            parent: taxonomy.parent,
            count: taxonomy.count,
        }
    }
}
