use crate::components::menu_item::MenuItem;
use entity::bowl::Model as Bowl;
use leptos::prelude::*;

#[component]
pub fn MenuList(bowls: Vec<Bowl>) -> impl IntoView {
    let bowls = RwSignal::new(bowls);

    view! {
        <div class="menu-list">
            <h1>"Our Menu"</h1>
            <div class="items-container">
                <For
                    each=move || bowls.get().into_iter()
                    key=|bowl| bowl.id
                    children=move |bowl| view! {
                        <MenuItem bowl=bowl />
                    }
                />
            </div>
        </div>
    }
}
