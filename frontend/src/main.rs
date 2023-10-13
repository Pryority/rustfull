// frontend/src/main.rs

mod api;
mod components;
mod store;
mod views;

use components::{
    alert::{AlertComponent, Props as AlertProps},
    product_form::ProductForm,
    // single_product::Product,
};
use store::Store;
use views::{products::Products, search_product::SearchProduct};
// use views::category_catalogue::CategoryCatelogue;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/admin")]
    Admin,
    #[at("/products")]
    Products,
    #[at("/product")]
    SearchProduct,
    #[at("/catelogue/:category")]
    CategoryCatelogue,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Admin => html! {
            <>
                <ProductForm/>
            </>
        },
        Route::SearchProduct => html! {
            <>
                <SearchProduct />
            </>
        },
        Route::Products => html! {
            <>
                <Products/>
            </>
        },
        Route::CategoryCatelogue => html! {
            <>
                // <CategoryCatelogue category_name={Route::Admin.into_prop_value()} list_title={"list title"} products={backend::get_product_handler(web::Path::into_inner(), )}/>
                // <Product title={"test"} sku={442}/>
            </>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let alert = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <BrowserRouter>
            if show_alert {
                <AlertComponent
                    message={alert.message}
                    delay_ms={alert.delay_ms}
                />
            }

            <main class="md:container mt-24 px-5">
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </main>
            if *loading {
                <div
                    class="fixed top-5 left-5 inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-yellow-400 border-r-transparent align-[-0.125em] text-warning motion-reduce:animate-[spin_1.5s_linear_infinite]"
                    role="status"
                >
                    <span class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]">
                        {"Loading..."}
                    </span>
                </div>
            }
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
