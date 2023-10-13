// use common::model::product::Product;
// use reqwasm::http::Request;
// use serde::Deserialize;
// use yew::prelude::*; // Import Deserialize from serde

// // use yew_oauth2::context::OAuth2Context;

// async fn get_catalogue(category: &String) -> CategoryCatelogue {
//     let url = format!("/api/catalogue/{}", category);
//     return Request::get(&url)
//         // .header("Authorization", token)
//         .send()
//         .await
//         // TODO: add error handling
//         .unwrap()
//         .json()
//         .await
//         .unwrap();
// }

// #[derive(Properties, Clone, PartialEq, Deserialize)]
// pub struct CategoryCatelogueViewProps {
//     pub category_name: String,
//     pub list_title: String,
//     pub products: Vec<Product>,
// }

// fn catelogue_item_to_html(product: &Product) -> Html {
//     let title_html = html! {<h1>{&product.title}</h1>};

//     html! {
//         <>
//             {title_html}
//             <h5>{product.sku}</h5>
//             <p>{product.description.clone()}</p>
//             <p>{product.category.clone()}</p>
//         </>
//     }
// }

// #[function_component(CategoryCatelogue)]
// pub fn product_list(props: &CategoryCatelogueViewProps) -> Html {
//     let category_name = props.category_name.clone();
//     let list_title = props.list_title.clone();
//     let products = use_state(Vec::new);
//     // let category = use_state(String::new);
//     {
//         let products = products.clone();
//         use_effect_with_deps(
//             move |_| {
//                 wasm_bindgen_futures::spawn_local(async move {
//                     let product_list = get_catalogue(&category_name).await;
//                     products.set(product_list.products);
//                 });
//             },
//             (),
//         );
//     }

//     let list_view = (*products).iter().map(|p| catelogue_item_to_html(p));

//     if list_view.len() > 0 {
//         return html! {
//             <>
//                 {list_view.collect::<Html>()}
//             </>
//         };
//     } else {
//         return html! {
//             <>
//                 <div>{"Loading..."}</div>
//             </>
//         };
//     }
// }
