use std::net::SocketAddr;
use rand::Rng;
use axum::{Router, routing::get};


fn pick_random_colour() -> String {
    let colours = ["red", "green", "blue","yellow","purple","orange","pink","brown","black","white"];
    return format!("The best colour is {}!", colours[rand::thread_rng().gen_range(0..colours.len())]);
}

fn init_router() -> Router {
    Router::new()
        .route("/colour", get(|| async { pick_random_colour() }))
}


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app: Router = init_router();
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}