use yew::prelude::*;

// use yew_oauth2::context::OAuth2Context;

#[function_component(Home)]
pub fn product_list() -> Html {
    return html! {
        <>
            <div class="flex flex-col h-full bg-purple-300/20 items-center gap-8 p-8 rounded-md">
            <h1 class="text-6xl text-white font-bold uppercase">{"My E-Commerce Store"}</h1>
            <a href="/admin" class="px-4 py-2 rounded-sm text-white bg-purple-800 border border-purple-500 shadow-sm">{"Manage Products"}</a>
            </div>
        </>
    };
}
