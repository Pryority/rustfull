use crate::api::{
    healthcheck::health_check,
    product::{create_product, delete_product, edit_product, get_product, get_products},
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(get_products)
        .service(create_product)
        .service(get_product)
        .service(edit_product)
        .service(delete_product);

    conf.service(scope);
}
