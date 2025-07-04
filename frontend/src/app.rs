use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub price: f64,
}

#[function_component(App)]
pub fn app() -> Html {
    // ... (прежнее содержимое компонента App)
    html! {
        <div>{"Working App"}</div>
    }
}