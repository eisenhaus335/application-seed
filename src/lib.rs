#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

type ID ;
struct Todo {
    id: Ulid,
    title: String,
    completed: bool
}

enum Filter {
    All,
    Active,
    Completed
}

struct SelectedTodo {
    id: ID,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}
struct Model {
    todos: Vec<Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    
}


#[derive(Copy, Clone)]
enum Msg {
    Increment,
}


fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]

fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}


#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
