use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::root)).service(
        web::scope("/api/v1")
            .route("/posts", web::get().to(handlers::get_posts))
            .route("/posts/{post_id}", web::get().to(handlers::get_post))
            .route(
                "/posts/{post_id}/meta",
                web::get().to(handlers::get_post_meta),
            )
            .route("/post-types", web::get().to(handlers::get_post_types))
            .route(
                "/post-types/{post_type}/posts",
                web::get().to(handlers::get_posts_by_type),
            )
            .route("/categories", web::get().to(handlers::get_categories))
            .route(
                "/categories/{category_id}/posts",
                web::get().to(handlers::get_posts_by_category),
            ),
    );
}