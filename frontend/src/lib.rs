use yew::prelude::*;
use gloo_net::http::Request;  // Версия 0.6.0
use serde::{Deserialize, Serialize};
use web_sys::console;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub price: f64,
}

#[function_component(App)]
pub fn app() -> Html {
    let items = use_state(Vec::<Item>::new);
    let error = use_state(|| None::<String>);

    use_effect_with((), {
        let items = items.clone();
        let error = error.clone();

        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/api/items").send().await {
                    Ok(response) => match response.json::<Vec<Item>>().await {
                        Ok(fetched_items) => {
                            items.set(fetched_items);
                            error.set(None);
                        },
                        Err(e) => {
                            error.set(Some(format!("JSON error: {}", e)));
                            console::error_1(&format!("JSON error: {}", e).into());
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Network error: {}", e)));
                        console::error_1(&format!("Network error: {}", e).into());
                    }
                }
            });
            || ()
        }
    });

    html! {
        <div class="shop">
            <h1>{"Anon Shop"}</h1>
            
            if let Some(err) = &*error {
                <div class="error">{err}</div>
            }
            
            <div class="items">
                {for items.iter().map(|item| html! {
                    <div class="item" key={item.id.clone()}>
                        <h3>{&item.name}</h3>
                        <p>{"Price: "}{item.price}{" XMR"}</p>
                        <button class="buy-button">{"Buy Anonymously"}</button>
                    </div>
                })}
            </div>
        </div>
    }
}