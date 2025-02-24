use entity::bowl::Model as Bowl;
use entity::customization::Model as Customization;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct CartItemData {
    pub bowl: Bowl,
    pub customizations: Vec<Customization>,
    pub quantity: i32,
}

#[component]
pub fn CartItem(item: CartItemData) -> impl IntoView {
    let base_price = item.bowl.price * (item.quantity as f32);

    view! {
        <div class="cart-item">
            <div class="item-row main-item">
                <span class="item-name">{item.bowl.name}</span>
                <span class="item-price">{"$"}{format!("{:.2}", item.bowl.price)}</span>
                <span class="item-quantity">{"×"}{item.quantity}</span>
                <span class="row-total">{"$"}{format!("{:.2}", base_price)}</span>
            </div>
            {
                if !item.customizations.is_empty() {
                    view! {
                        <div class="customizations">
                            <For
                                each=move || item.customizations.clone().into_iter()
                                key=|c| c.id
                                children=move |c| {
                                    let row_total = c.price * (item.quantity as f32);
                                    view! {
                                        <div class="item-row sub-item">
                                            <span class="item-name">{"+ "}{c.name}</span>
                                            <span class="item-quantity">{"x"}{item.quantity}</span>
                                            <span class="item-price">{"$"}{format!("{:.2}", c.price)}</span>
                                            <span class="row-total">{"$"}{format!("{:.2}", row_total)}</span>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }
        </div>
    }
}
