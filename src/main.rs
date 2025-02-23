use migration::{Migrator, MigratorTrait};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::{log, warn};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_seaorm_practice_bowl_shop::app::*;

    let env_result = dotenvy::dotenv();
    if env_result.is_err() {
        warn!("No .env file found");
    }

    let connection =
        sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap_or_default()).await;
    match connection {
        Ok(connection) => {
            if let Err(e) = Migrator::up(&connection, None).await {
                warn!("Error migrating database: {:?}", e);
                return;
            }
        }
        Err(e) => {
            warn!("Error connecting to database: {:?}", e);
            return;
        }
    };

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
