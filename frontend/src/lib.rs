use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

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

    {
        let items = items.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let items = items.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/api/items").send().await {
                    Ok(response) => {
                        match response.json::<Vec<Item>>().await {
                            Ok(fetched_items) => {
                                items.set(fetched_items);
                                error.set(None);
                            },
                            Err(e) => {
                                error.set(Some(format!("Failed to parse items: {}", e)));
                            }
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch items: {}", e)));
                    }
                }
            });

            || ()
        });
    }

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

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}