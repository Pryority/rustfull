use crate::components::single_product::SingleProduct;
use common::model::product::Product;
use serde::Serialize;
use uuid::Uuid;
use yew::prelude::*;


#[function_component(SearchProduct)]
pub fn search_product() -> Html {
    // let result: Result<Product, sqlx::Error> = query_as!(
    //     Product,
    //     "SELECT id, title, description FROM products WHERE title = $1 AND quantity = $2",
    //     title,
    //     quantity
    // );

    return html! {
        <>
            <section>
            // <SingleProduct product={product} />
            </section>
        </>
    };
}
