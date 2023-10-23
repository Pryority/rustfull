use crate::{
    api::product::create_product,
    store::{set_loading, set_product, set_show_alert, Store},
};

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Product {
    pub title: String,
    pub description: String,
    pub sku: i32,
    pub category: String,
    pub quantity: i32,
    pub price: i32,
    pub sale_price: i32,
    pub on_sale: bool,
}

#[function_component]
pub fn ProductForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    // let loading = &store.loading;
    let title = use_state(String::new);
    let description = use_state(String::new);
    let sku = use_state(|| 0);
    let category = use_state(String::new);
    let quantity = use_state(|| 0);
    let price = use_state(|| 0);
    let sale_price = use_state(|| 0);
    let on_sale = use_state(|| false);

    let title_input_ref = use_node_ref();
    let desc_input_ref = use_node_ref();
    let sku_input_ref = use_node_ref();
    let category_select_ref = use_node_ref();
    let quantity_input_ref = use_node_ref();
    let price_input_ref = use_node_ref();
    let sale_price_input_ref = use_node_ref();
    let on_sale_select_ref = use_node_ref();

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

    let handle_category_change = {
        let category = category.clone();
        Callback::from(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(select) = target.dyn_into::<HtmlInputElement>() {
                    let value = select.value();
                    category.set(value);
                }
            }
        })
    };

    let handle_quantity_input = {
        let quantity = quantity.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>();
            quantity.set(value.unwrap());
        })
    };

    let handle_price_input = {
        let price = price.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>();
            price.set(value.unwrap());
        })
    };

    let handle_sale_price_input = {
        let sale_price = sale_price.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>();
            sale_price.set(value.unwrap());
        })
    };

    let handle_on_sale_change = {
        let on_sale = on_sale.clone();
        Callback::from(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    on_sale.set(input.checked());
                }
            }
        })
    };

    let submit = {
        let cloned_dispatch = dispatch.clone();
        let cloned_title_input_ref = title_input_ref.clone();
        let cloned_title = title.clone();
        let cloned_desc_input_ref = desc_input_ref.clone();
        let cloned_description = description.clone();
        let cloned_sku_input_ref = sku_input_ref.clone();
        let cloned_sku = sku.clone();
        let cloned_category_input_ref = category_select_ref.clone();
        let cloned_category = category.clone();
        let cloned_quantity_input_ref = quantity_input_ref.clone();
        let cloned_quantity = quantity.clone();
        let cloned_price_input_ref = price_input_ref.clone();
        let cloned_price = price.clone();
        let cloned_sale_price_input_ref = sale_price_input_ref.clone();
        let cloned_sale_price = sale_price.clone();
        let cloned_on_sale_select_ref = on_sale_select_ref.clone();
        let cloned_on_sale = on_sale.clone();

        Callback::from(move |event: SubmitEvent| {
            let dispatch = cloned_dispatch.clone();
            let title_input_ref = cloned_title_input_ref.clone();
            let title = cloned_title.clone();
            let desc_input_ref = cloned_desc_input_ref.clone();
            let description = cloned_description.clone();
            let sku_input_ref = cloned_sku_input_ref.clone();
            let sku = cloned_sku.clone();
            let category_input_ref = cloned_category_input_ref.clone();
            let category = cloned_category.clone();
            let quantity_input_ref = cloned_quantity_input_ref.clone();
            let quantity = cloned_quantity.clone();
            let price_input_ref = cloned_price_input_ref.clone();
            let price = cloned_price.clone();
            let sale_price_input_ref = cloned_sale_price_input_ref.clone();
            let sale_price = cloned_sale_price.clone();
            let on_sale_select_ref = cloned_on_sale_select_ref.clone();
            let on_sale = cloned_on_sale.clone();

            event.prevent_default();
            set_loading(true, dispatch.clone());

            if title.is_empty() {
                // Show an error message or perform any other validation logic here
                set_show_alert("Title is required!".to_string(), dispatch.clone());
                set_loading(false, dispatch.clone());
                return;
            }
            if description.is_empty() {
                // Show an error message or perform any other validation logic here
                set_show_alert("Description is required!".to_string(), dispatch.clone());
                set_loading(false, dispatch.clone());
                return;
            }
            if sku.abs() == 0 {
                // Show an error message or perform any other validation logic here
                set_show_alert("SKU is required!".to_string(), dispatch.clone());
                set_loading(false, dispatch.clone());
                return;
            }
            if quantity.abs() == 0 {
                // Show an error message or perform any other validation logic here
                set_show_alert("Quantity is required!".to_string(), dispatch.clone());
                set_loading(false, dispatch.clone());
                return;
            }
            if price.abs() == 0 {
                // Show an error message or perform any other validation logic here
                set_show_alert("Price is required!".to_string(), dispatch.clone());
                set_loading(false, dispatch.clone());
                return;
            }
            if sale_price.abs() == 0 {
                // Show an error message or perform any other validation logic here
                set_show_alert("Sale Price is required!".to_string(), dispatch.clone());
                set_loading(false, dispatch.clone());
                return;
            }

            let product_data = serde_json::json!({
                "title": title.to_string(),
                "description": description.to_string(),
                "sku": *sku,
                "category": category.to_string(),
                "quantity": *quantity,
                "price": *price,
                "sale_price": *sale_price,
                "on_sale": *on_sale,
            });

            spawn_local(async move {
                set_loading(true, dispatch.clone());
                let title_input = title_input_ref.cast::<HtmlInputElement>().unwrap();
                title_input.set_value("");
                title.set(String::new());
                let desc_input = desc_input_ref.cast::<HtmlInputElement>().unwrap();
                desc_input.set_value("");
                description.set(String::new());
                let sku_input = sku_input_ref.cast::<HtmlInputElement>().unwrap();
                sku_input.set_value("");
                sku.set(0);
                let category_input = category_input_ref.cast::<HtmlInputElement>().unwrap();
                category_input.set_value("");
                category.set(String::new());
                let quantity_input = quantity_input_ref.cast::<HtmlInputElement>().unwrap();
                quantity_input.set_value("");
                quantity.set(0);
                let price_input = price_input_ref.cast::<HtmlInputElement>().unwrap();
                price_input.set_value("");
                price.set(0);
                let sale_price_input = sale_price_input_ref.cast::<HtmlInputElement>().unwrap();
                sale_price_input.set_value("");
                sale_price.set(0);
                let on_sale_input = on_sale_select_ref.cast::<HtmlInputElement>().unwrap();
                on_sale_input.set_value("");
                on_sale.set(false);

                let response = create_product(product_data.to_string().as_str()).await;

                match response {
                    Ok(product) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert("Product added successfully".to_string(), dispatch.clone());
                        set_product(product, dispatch);
                        reset_form_fields(
                            &title,
                            &description,
                            &sku,
                            &category,
                            &quantity,
                            &price,
                            &sale_price,
                            &on_sale,
                        );
                    }
                    Err(e) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            });
        })
    };

    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <header class="flex w-full">
                <h2 class="text-2xl font-bold">{"Product Creation"}</h2>
            </header>

            <form onsubmit={submit} class="flex flex-col items-center gap-8 p-4 rounded-md bg-white">
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
                <label class="flex flex-col w-full justify-center">
                        <span class="text-xs tracking-wide text-gray-500 mb-1">{"Select a Category"}</span>
                    <select
                        placeholder="Category"
                        ref={category_select_ref.clone()}
                        onchange={handle_category_change}
                        class="w-full border p-1"
                    >
                    <option disabled=true selected=true hidden=true>{"---"}</option>
                    <option value="clothing">{"Clothing"}</option>
                    <option value="books">{"Books"}</option>
                    <option value="electronics">{"Electronics"}</option>
                    </select>
                </label>
                <input
                type="number"
                placeholder="Quantity (#)"
                ref={quantity_input_ref}
                oninput={handle_quantity_input}
                class="w-full border p-1"
                />
                <input
                type="number"
                placeholder="Price ($)"
                ref={price_input_ref}
                oninput={handle_price_input}
                class="w-full border p-1"
                />
                <input
                type="number"
                placeholder="Sale Price ($)"
                ref={sale_price_input_ref}
                oninput={handle_sale_price_input}
                class="w-full border p-1"
                />
                <label class="flex w-full gap-8 items-center">
                    {"Is this product on sale?"}
                    <input
                        type="checkbox"
                        ref={on_sale_select_ref}
                        onchange={handle_on_sale_change}
                        class="border p-1 h-8 w-8"
                    />
                </label>

                <button
                    type="submit"
                    class="px-4 py-2 rounded-lg bg-violet-700 text-violet-50 border-4 border-violet-400"
                >{"Create"}</button>
            </form>
        </div>
    }
}

fn reset_form_fields(
    title: &UseStateHandle<String>,
    description: &UseStateHandle<String>,
    sku: &UseStateHandle<i32>,
    category: &UseStateHandle<String>,
    quantity: &UseStateHandle<i32>,
    price: &UseStateHandle<i32>,
    sale_price: &UseStateHandle<i32>,
    on_sale: &UseStateHandle<bool>,
) {
    title.set(String::new());
    description.set(String::new());
    sku.set(0);
    category.set(String::new());
    quantity.set(0);
    price.set(0);
    sale_price.set(0);
    on_sale.set(false);
}
