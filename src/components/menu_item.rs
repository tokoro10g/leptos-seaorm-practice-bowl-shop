use entity::bowl::Model as Bowl;
use leptos::prelude::*;

#[component]
pub fn MenuItem(bowl: Bowl) -> impl IntoView {
    let active_bowl =
        use_context::<RwSignal<Option<Bowl>>>().expect("active_bowl should be provided");

    let bowl_2 = bowl.clone();
    let open_popup = move |_| {
        active_bowl.set(Some(bowl_2.clone()));
    };
    view! {
        <div class="bowl-item">
            <h2>{bowl.name}</h2>
            <p>{format!("{:.2}",bowl.price)}</p>
            <button class="add-to-cart-button" on:click=open_popup>
                "Add to Cart"
            </button>
        </div>
    }
}
