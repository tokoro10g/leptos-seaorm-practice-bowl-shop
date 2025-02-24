use crate::components::cart_item::CartItemData;
use entity::bowl::Model as Bowl;
use entity::customization::Model as Customization;
use leptos::prelude::*;

#[component]
fn CustomizationsList(
    customizations: Vec<Customization>,
    selected: Signal<Vec<Customization>>,
    on_toggle: Callback<Customization>,
) -> impl IntoView {
    if customizations.is_empty() {
        view! { <div class="customizations-list"/> }.into_any()
    } else {
        view! {
            <div class="customizations-list">
                <h4>"Customizations"</h4>
                <For
                    each=move || customizations.clone()
                    key=|c| c.id
                    children=move |c| {
                        let c2 = c.clone();
                        view! {
                            <label>
                                <input
                                    type="checkbox"
                                    checked=selected.get().iter().any(|x| x.id == c.id)
                                    on:change=move |_| on_toggle.run(c2.clone())
                                />
                                {move || c.name.clone()}
                                {move || format!(" (${:.2})", c.price)}
                            </label>
                        }
                    }
                />
            </div>
        }
        .into_any()
    }
}

#[component]
pub fn AddToCartPopup(
    bowl: RwSignal<Option<Bowl>>,
    customizations: Resource<Result<Vec<Customization>, ServerFnError>>,
) -> impl IntoView {
    let (quantity, set_quantity) = signal(1);
    let (selected_customizations, set_selected_customizations) = signal(Vec::new());
    let active_bowl =
        use_context::<RwSignal<Option<Bowl>>>().expect("active_bowl should be provided");
    let add_to_cart =
        use_context::<Callback<CartItemData>>().expect("AddToCartFn context not found");

    let toggle_customization = move |c: Customization| {
        set_selected_customizations.update(|list| {
            if let Some(pos) = list.iter().position(|x: &Customization| x.id == c.id) {
                list.remove(pos);
            } else {
                list.push(c);
            }
        });
    };

    let handle_add = move |_| {
        if let Some(bowl) = bowl.get() {
            add_to_cart.run(CartItemData {
                bowl,
                customizations: selected_customizations.get(),
                quantity: quantity.get(),
            });
            active_bowl.set(None);
            set_quantity.set(1);
            set_selected_customizations.set(Vec::new());
        }
    };

    let bowl_valid = Signal::derive(move || bowl.get().is_some());

    view! {
        <div class="popup-overlay" class:open=bowl_valid>
            <div class="add-to-cart-popup">
                <button class="close-button" on:click=move |_| active_bowl.set(None)>
                    "×"
                </button>
                <h2>"Add to Cart"</h2>
                {move || bowl.get().map(|b| view! {
                    <>
                        <h3>{move|| b.name.clone()}</h3>
                        <p class="base-price">
                            {format!("Base price: ${:.2}", b.price)}
                        </p>
                    </>
                })}
                <Suspense fallback=move || view! { <div>"Loading..."</div> }>
                    {move || customizations.get().map(|c| match c {
                        Ok(list) => view! {
                            <CustomizationsList
                                customizations=list
                                selected=selected_customizations.into()
                                on_toggle=Callback::new(toggle_customization)
                            />
                        }.into_any(),
                        Err(_) => view! { <div>"Error loading customizations"</div> }.into_any()
                    })}
                </Suspense>
                <div class="quantity-section">
                    <label>
                        "Quantity: "
                        <input
                            type="number"
                            min="1"
                            max="99"
                            value=quantity
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                    set_quantity.set(val.clamp(1, 99));
                                }
                            }
                        />
                    </label>
                </div>
                <button class="add-button" on:click=handle_add>
                    "Add to Cart"
                </button>
            </div>
        </div>
    }
}
