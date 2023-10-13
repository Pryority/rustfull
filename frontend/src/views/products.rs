use yew::prelude::*;

use crate::components::single_product::SingleProduct;
use common::model::product::Product;

#[derive(Properties, Clone, PartialEq)]
pub struct ProductProps {
    #[prop_or_default]
    pub products: Vec<Product>,
}


#[function_component(Products)]
pub fn products(props: &ProductProps) -> Html {
    // let get_request = Request::get("http://localhost:8000/api/products")
    // .body()
    // .expect("Could not build that request");
    return html! {
        <>
            <section>
            { for props.products.iter().map(|product| {
                html! { <SingleProduct product={product.clone()} /> }
            })}
            </section>
        </>
    };
}
