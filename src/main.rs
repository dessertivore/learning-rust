use axum::{extract::Json, routing::get, routing::post, Router};
use rand::Rng;
use serde::Deserialize;
use std::net::SocketAddr;
use log::info;

async fn pick_random_colour() -> String {
    let colours = [
        "red", "green", "blue", "yellow", "purple", "orange", "pink", "brown", "black", "white",
    ];
    return format!(
        "The best colour is {}!",
        colours[rand::thread_rng().gen_range(0..colours.len())]
    );
}

#[derive(Deserialize)]
struct NumList {
    nums: String,
}

async fn add_nums(Json(payload): Json<NumList>) -> String {
    let sum: i32 = payload.nums
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap_or(0))
        .sum();
    println!("The sum of the numbers is {}", sum);
    return format!("The sum of the numbers is {}", sum);
}

fn init_router() -> Router {
    return 
    Router::new().
    route("/colour", get(|| pick_random_colour()))
    .route("/addition", post(add_nums));
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app: Router = init_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
