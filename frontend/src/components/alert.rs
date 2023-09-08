use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::{set_hide_alert, Store};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub message: String,
    pub delay_ms: u32,
}

#[function_component]
pub fn AlertComponent(props: &Props) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let show_alert = store.alert_input.show_alert;

    use_effect_with_deps(
        move |(show_alert, dispatch, delay_ms)| {
            let cloned_dispatch = dispatch.clone();
            if *show_alert {
                let handle =
                    Timeout::new(*delay_ms, move || set_hide_alert(cloned_dispatch)).forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap(),
                        handle.as_f64().unwrap() as i32,
                    );
                };

                Box::new(clear_handle) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        },
        (show_alert, dispatch.clone(), props.delay_ms),
    );

    html! {
    <div id="myToast" class={format!("fixed top-14 right-10 px-5 py-4 border-r-8 border-lime-500 bg-white drop-shadow-lg {}",
        if props.message == "That SKU already exists!"
        || props.message == "Title is required!"
        || props.message == "Description is required!"
        || props.message == "SKU is required!"
        || props.message == "Quantity is required!"
        || props.message == "Price is required!"
        || props.message == "Sale Price is required!"
        { "border-red-500" }
        else if show_alert {""}
        else { "hidden" })}
    >
    <p class="text-sm">
        <span class="mr-2 inline-block px-3 py-1 rounded-full bg-blue-500 text-white font-extrabold">{"i"}</span>
        {props.message.clone()}
    </p>
    </div>
    }
}
