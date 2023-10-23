use crate::api::product::get_all;
use common::model::product::Product;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub enum Msg {
    FetchProducts,
    ProductsFetched(Result<Vec<Product>, String>),
}

#[function_component(ProductList)]
pub fn product_list() -> Html {
    let products_raw = use_state(|| Vec::new());
    let is_loading_raw = use_state(|| false);

    // Create state handles explicitly
    let products: UseStateHandle<Vec<Product>> = products_raw.into();
    let is_loading: UseStateHandle<bool> = is_loading_raw.into();

    let callback = Callback::from(move |response: Result<Vec<Product>, String>| {
        is_loading.set(false);
        match response {
            Ok(product_list) => {
                products.set(product_list);
            }
            Err(error) => {
                // Handle the error, for example, by displaying an error message
                log::error!("Failed to fetch products: {}", error);
            }
        }
    });

    use_effect(move || {
        let is_loading_clone = is_loading.clone();
        is_loading_clone.set(true);
        spawn_local(async move {
            let result = get_all().await;
            callback.emit(result);
        });

        || {
            // Cleanup logic, if needed
        }
    });

    html! {
        <div class="flex flex-col gap-4 bg-white p-4 rounded-md border h-fit">
            { if *is_loading {
                html! { <p>{"Loading..."}</p> }
            } else {
                html! { for products.iter().map(render_product) }
            }}
        </div>
    }
}

fn render_product(product: &Product) -> Html {
    html! {
        <div class="flex w-full h-full bg-slate-200 z-50">
            <h3 class="font-semibold text-xl">{ &product.title }</h3>
            <p>{ &product.description }</p>
            <p>{ format!("SKU: {}", &product.sku) }</p>
        </div>
    }
}
