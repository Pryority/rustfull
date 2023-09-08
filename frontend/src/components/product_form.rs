use crate::{
    api::api_create_product,
    store::{set_loading, set_product, set_show_alert, Store},
};

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn ProductForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let title = use_state(String::new);
    let description = use_state(String::new);
    let sku: UseStateHandle<i32> = use_state(|| 0);
    let category = use_state(String::new);
    let quantity = use_state(|| 0);
    let price = use_state(|| 0);
    let sale_price = use_state(|| 0);
    let on_sale = use_state(|| false);

    let title_input_ref = use_node_ref();
    let desc_input_ref = use_node_ref();
    let sku_input_ref = use_node_ref();

    let handle_title_input = {
        let title = title.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            title.set(value);
        })
    };

    let handle_description_input = {
        let description = description.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            description.set(value);
        })
    };

    let handle_sku_input = {
        let sku = sku.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>();
            sku.set(value.unwrap());
        })
    };

    html! {
        <div class="flex flex-col items-center gap-8 p-4 rounded-md bg-white">
            <input
                type="text"
                placeholder="Title"
                ref={title_input_ref}
                oninput={handle_title_input}
                class="w-full border p-1"
            />
            <input
                type="text"
                placeholder="Description"
                ref={desc_input_ref}
                oninput={handle_description_input}
                class="w-full border p-1"
            />
            <input
            type="number"
            placeholder="SKU"
            ref={sku_input_ref}
            oninput={handle_sku_input}
            class="w-full border p-1"
            />
            <input
            type="select"
            placeholder="Category"
            // ref={desc_input_ref}
            // oninput={handle_description_input}
            class="w-full border p-1"
            />
            <input
            type="number"
            placeholder="Price ($)"
            // ref={sku_input_ref}
            // oninput={handle_sku_input}
            class="w-full border p-1"
            />
            <input
            type="number"
            placeholder="Sale Price ($)"
            // ref={sku_input_ref}
            // oninput={handle_sku_input}
            class="w-full border p-1"
            />
            <input
            type="radio"
            placeholder="Is this product on sale?"
            // ref={sku_input_ref}
            // oninput={handle_sku_input}
            class="w-full border p-1"
            />

            <button >{"Submit"}</button>
        </div>
    }
}
