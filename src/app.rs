use crate::components::cart_tooltip::CartTooltip;
use crate::components::menu_list::MenuList;
use crate::components::{add_to_cart_popup::AddToCartPopup, cart_item::CartItemData};
use crate::server::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn Navigation(
    cart_num: impl Fn() -> usize + Send + Sync + 'static,
    cart_open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <nav class="main-nav">
            <div class="nav-container">
                <a href="/" class="logo">"Bowl Shop"</a>
                <div class="nav-links">
                    <button
                        class="cart-button"
                        on:click=move |_| cart_open.update(|open| *open = !*open)
                    >
                        "Cart"
                        <span class="cart-count">{cart_num}</span>
                    </button>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn MenuPage() -> impl IntoView {
    let bowls = Resource::new(|| (), |_| get_bowls());
    let active_bowl = RwSignal::<Option<entity::bowl::Model>>::new(None);

    provide_context(active_bowl);

    view! {
        <AddToCartPopup
            bowl=active_bowl
            customizations=Resource::new(
                move || active_bowl.get(),
                move |bowl| async move {
                    if let Some(bowl) = bowl {
                        get_customizations(bowl).await
                    } else {
                        std::future::ready(Ok(Vec::new())).await
                    }
                }
            )
        />
        <Suspense fallback=move || view!{ <div><p>"Loading..."</p></div> }>
            {move || {
                bowls.get().map(|result| {
                    match result {
                        Ok(bowls) => view! { <MenuList bowls/> }.into_any(),
                        Err(e) => view! { <p>"Error loading menu: " {e.to_string()}</p> }.into_any(),
                    }
                })
            }}
        </Suspense>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (cart_items, set_cart_items) = signal(Vec::new());
    let cart_open = RwSignal::new(false);

    let add_to_cart = Callback::new(move |new_item: CartItemData| {
        set_cart_items.update(|items| {
            let mut updated_items = items.clone();
            if let Some(existing_item) =
                updated_items.iter_mut().find(|item: &&mut CartItemData| {
                    item.bowl.id == new_item.bowl.id
                        && item.customizations.len() == new_item.customizations.len()
                        && item
                            .customizations
                            .iter()
                            .all(|c1| new_item.customizations.iter().any(|c2| c1.id == c2.id))
                })
            {
                existing_item.quantity += new_item.quantity;
            } else {
                updated_items.push(new_item);
            }
            *items = updated_items;
        });
        cart_open.set(true);
    });
    provide_context::<Callback<CartItemData>>(add_to_cart);

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-seaorm-practice-bowl-shop.css"/>
        <Title text="Bowl Shop"/>

        <Router>
            <Navigation cart_open cart_num=move|| cart_items.get().len()/>
            <CartTooltip
                cart_items=cart_items
                is_open=cart_open
            />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=MenuPage/>
                </Routes>
            </main>
        </Router>
    }
}
