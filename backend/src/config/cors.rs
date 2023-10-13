use actix_cors::Cors;
use actix_web::http::header;

pub fn config() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}
