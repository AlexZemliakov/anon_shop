use yew::prelude::*;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, serde::Deserialize)]
struct Item {
    id: String,
    name: String,
    price: f64,
}

#[function_component(App)]
fn app() -> Html {
    let items = use_state(Vec::<Item>::new);

    {
        let items = items.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_items: Vec<Item> = Request::get("/items")
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
                    <div class="item">
                        <h3>{&item.name}</h3>
                        <p>{"Price: "}{item.price}</p>
                        <button>{"Buy"}</button>
                    </div>
                })}
            </div>
        </div>
    }
}