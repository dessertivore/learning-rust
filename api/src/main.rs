use axum::{extract::Json, routing::get, routing::post, Router};
use rand::Rng;
use serde::Deserialize;
use tokio::{task::{self, JoinHandle}, time::{sleep,Duration}};
use std::net::SocketAddr;

/// This function picks a random colour from a list of colours and returns it as a string.
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

/// This function takes a list of numbers as a JSON payload and returns the sum of 
/// the numbers, but first, it spawns 20 async tasks that each sleep for 2 seconds.
/// This is just an exercise to demonstrate how async tasks work in Rust - the 
/// 'sleep' tasks will run in parallel and the total time taken to complete the
/// function will be around 2 seconds.
async fn add_nums(Json(payload): Json<NumList>) -> String {
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let now = tokio::time::Instant::now();
    for i in 0..20 {
        let handle = task::spawn(async move {
            sleep(Duration::from_secs(2)).await;
            println!("2 seconds have now passed {} times", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }

    let sum: i32 = payload.nums
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap_or(0))
        .sum();
    println!("The sum of the numbers is {}", sum);
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", elapsed);
    return format!("The sum of the numbers is {}", sum);
}

/// This function initializes our router with two routes: one for the colour endpoint
/// and one for the addition endpoint.
fn init_router() -> Router {
    return 
    Router::new().
    route("/colour", get(|| pick_random_colour()))
    .route("/addition", post(add_nums));
}

/// This is the main function that starts our server.
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app: Router = init_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
