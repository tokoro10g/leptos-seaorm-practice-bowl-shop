use crate::components::cart_item::{CartItem, CartItemData};
use leptos::prelude::*;

#[component]
pub fn CartTooltip(
    cart_items: ReadSignal<Vec<CartItemData>>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    let grand_total = Memo::new(move |_| {
        cart_items
            .get()
            .iter()
            .map(|item| {
                let base_price = item.bowl.price * (item.quantity as f32);
                let customization_total: f32 = item
                    .customizations
                    .iter()
                    .map(|c| c.price * (item.quantity as f32))
                    .sum();
                base_price + customization_total
            })
            .sum::<f32>()
    });

    let sorted_items = move || {
        let mut items = cart_items.get();
        items.sort_by_key(|item| item.bowl.id);
        items
    };

    view! {
        <div class="cart-tooltip" class:open=is_open>
            <button class="close-button" on:click=move |_| is_open.set(false)>
                "×"
            </button>
            <h2>"Your Cart"</h2>
            <div class="cart-items-list">
                <For
                    each=sorted_items
                    key=|item| (item.bowl.id, item.customizations.len())
                    children=move |item| view! { <CartItem item /> }
                />
            </div>
            <div class="cart-footer">
                <div class="grand-total">
                    <span class="total-label">"Total"</span>
                    <span class="total-amount">
                        {"$"}{move || format!("{:.2}", grand_total.get())}
                    </span>
                </div>
                <button class="checkout-button">"Proceed to Checkout"</button>
            </div>
        </div>
    }
}
