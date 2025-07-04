use yew::prelude::*;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, serde::Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub price: f64,
}

#[function_component(App)]
pub fn app() -> Html {
    let items = use_state(Vec::<Item>::new);

    {
        let items = items.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_items: Vec<Item> = Request::get("/api/items")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                items.set(fetched_items);
            });
            || ()
        }, ());
    }

    html! {
        <div class="shop">
            <h1>{"Anon Shop"}</h1>
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