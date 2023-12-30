use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::json;
use web_sys::{
    window,
    HtmlInputElement,
    wasm_bindgen::JsCast
};

#[derive(Default, PartialEq)]
struct NewBook {
    title: String,
    is_owned: bool,
}

#[function_component(AddBookButton)]
pub fn add_book() -> Html {
    let on_click = Callback::from(move |_| {
        let title = get_value_by_id("title").parse::<String>().unwrap();
        let is_owned = get_value_by_id("is_owned").parse::<bool>().unwrap();
        wasm_bindgen_futures::spawn_local(async move {
            Request::post("http://127.0.0.1:3000/book/create")
                .header("Content-Type", "application/json")
                .json(&json!({"title": title, "is_owned": is_owned }))
                .expect("lol")
                .send()
                .await
                .unwrap();
        });
    });
    return html! {
        <div>
            <p>{"title"}<input type="text" id="title" /></p>
            <p>{"owned?? true or false"}<input type= "boolean" id="is_owned" /></p>
            <button onclick={on_click}>{ "ADD BOOK" }</button>
        </div>
    }
}


fn get_value_by_id(id: &str) -> String {
    window().unwrap()
    .document().unwrap()
    .get_element_by_id(id).unwrap()
    .dyn_ref::<HtmlInputElement>().unwrap()
    .value()
}