use yew::prelude::*;

use crate::Book;

#[derive(Properties, PartialEq)]
pub struct BooksListProps {
    pub books: Vec<Book>,
}

#[function_component(BooksList)]
pub fn books_list(BooksListProps { books }: &BooksListProps) -> Html {
    books.iter().map(|book| html! {
        <p key={book.id}>{format!("{}: {}", book.title, if book.is_owned {"I have this book."} else {"I have already trust it."})}</p>
    }).collect()
}