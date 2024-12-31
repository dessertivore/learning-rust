use axum::{ extract::Json, routing::{get, post}, Extension, Router};
use rand::Rng;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use tokio::{task::{self, JoinHandle}, time::{sleep,Duration}, sync::Mutex};
use std::{cell::{Cell, UnsafeCell}, clone, marker::PhantomData, net::SocketAddr, sync::{atomic::AtomicUsize, Arc}};
use axum_macros::debug_handler;


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

// #[derive(Copy, Clone, Debug, Default)]
// struct NotSyncMarker(PhantomData<Cell<*mut usize>>);

#[derive(Serialize, Deserialize, clone::Clone)]
struct Counter {
    count: usize,
}

struct UnsafeCounter {
    counter: UnsafeCell<usize>,
}

impl Clone for UnsafeCounter {
    fn clone(&self) -> UnsafeCounter {
        UnsafeCounter {
            counter: UnsafeCell::new(unsafe { *self.counter.get() }),
        }
    }
}


impl Serialize for UnsafeCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let count = unsafe { *self.counter.get() };
        let mut state = serializer.serialize_struct("Counter", 1)?;
        state.serialize_field("count", &count)?;
        state.end()
    }
}

unsafe impl Sync for UnsafeCounter {}

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

// async fn increment_counter(Extension(counter): Extension<Arc<Mutex<Counter>>>) -> Json<Counter> {
#[debug_handler]
async fn increment_counter(counter: Extension<Arc<UnsafeCounter>>) -> Json<UnsafeCounter> {
    let counter_ref = unsafe { &mut *counter.counter.get() };
    *counter_ref += 1;
    return Json(UnsafeCounter { counter: UnsafeCell::new(*counter_ref) })
}

async fn check_counter(counter: Extension<Arc<UnsafeCounter>>) -> () {
    let counter_ref = unsafe { &mut *counter.counter.get() };
    print!("{:?}", Json(counter_ref));
}

/// This function initializes our router with two routes: one for the colour endpoint
/// and one for the addition endpoint.
fn init_router() -> Router {
    let counter = Arc::new(UnsafeCounter{counter: UnsafeCell::new(0)});
    return 
    Router::new()
    .route("/colour", get(|| pick_random_colour()))
    .route("/counter", get({
        let counter = counter.clone();
        move || check_counter(Extension(counter))
    }))
    .route("/counter", post({
        let counter = counter.clone();
        move || increment_counter(Extension(counter))
    }))
    .route("/addition", post(add_nums));
}

/// This is the main function that starts our server.
#[tokio::main]
async fn main() {
    
    // build our application with a Router with multiple routes
    let app: Router = init_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
