use gloo_net::http::Request;
use yew::prelude::*;
use serde::Deserialize;

mod components;

use components::books_list::BooksList;
use components::add_book::AddBookButton;

#[function_component(App)]
fn app() -> Html {
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with((), move |_| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_books: Vec<Book> = Request::get("http://127.0.0.1:3000/book/list")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                books.set(fetched_books);
            });
            || ()
        });
    };


    return html! {
        <div>
            <BooksList books={(*books).clone()} />
            <AddBookButton />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


#[derive(Clone, PartialEq, Deserialize)]
struct Book {
    id: usize,
    title: String,
    is_owned: bool,
}