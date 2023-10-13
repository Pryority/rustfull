// frontend/src/components/single_product.rs

use common::model::product::Product;
// use common::model::product::ProductData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProductProps {
    pub product: Product,
}

#[function_component(SingleProduct)]
pub fn product_view(p: &ProductProps) -> Html {
    let title_html = html! {<p>{p.product.title.clone()}</p>};
    let description_html = html! {<p>{p.product.description.clone()}</p>};
    let sku_html = html! {<p>{p.product.sku.clone()}</p>};
    let category_html = html! {<p>{p.product.category.clone()}</p>};
    let quantity_html = html! {<p>{p.product.quantity.clone()}</p>};
    let price_html = html! {<p>{p.product.price.clone()}</p>};
    let sale_price_html = html! {<p>{p.product.sale_price.clone()}</p>};
    let on_sale_html = html! {<p>{p.product.on_sale.clone()}</p>};

    html! {
        <>
            <div class="flex flex-col gap-4 bg-white p-4 rounded-md border">
                {title_html}
                {sku_html}
                {description_html}
                {category_html}
                {quantity_html}
                {price_html}
                {sale_price_html}
                {on_sale_html}
            </div>
        </>
    }
}
