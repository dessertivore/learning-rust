use reqwest::{self};
use serde::Serialize;
use tokio::task::{self, JoinHandle};

#[derive(Serialize, Clone)]
struct NumList {
    nums: String,
}

/// This function makes a post request to the server with a payload of numbers, and returns
/// the response as a string.
async fn send_payload(payload: NumList) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8000/addition")
        .json(&payload)
        .send()
        .await
        .unwrap();
    return resp.text().await.unwrap();
}

async fn call_addition_endpoint() {
    let body_for_post = NumList {
        nums: "1,2,3,4".to_string(),
    };

    // Call addition endpoint and print response to console
    // This is asynchronously done 20 times, so will take 2 seconds to complete
    // (endpoint takes 2 seconds to return request, and requests are made asynchronously
    // so are not blocking)
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let now = tokio::time::Instant::now();
    
    for i in 0..20 {
        let body_for_post_clone = body_for_post.clone();
        let handle = task::spawn(async move {
            let sum = send_payload(body_for_post_clone).await;
            println!("Response from /addition: {}. Endpoint has bee called {} times.", sum,i);
        });
        handles.push(handle);
    }
    // Await all the handles to complete
    for handle in handles {
        handle.await.unwrap();
    }
    println!("Time elapsed: {:?}", now.elapsed());

}

async fn inc_counter() -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8000/counter")
        .json("")
        .send()
        .await
        .unwrap();
    return resp.text().await.unwrap();
}
async fn check_counter() -> String {
    let resp = reqwest::get("http://localhost:8000/counter").await;
    let body = resp.unwrap().text().await.unwrap();
    return body;
}
/// This is the main function that makes a get request, and then 20 post requests to 
/// the server in order to test async functionality.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Call colour endpoint and print response to console
    let resp = reqwest::get("http://localhost:8000/colour").await?;
    let body = resp.text().await?;
    println!("Response: {}", body);

    // Call addition endpoint and print response to console
    // call_addition_endpoint().await;
    let mut handles = vec![];
    for i in 0..10000 {
        let handle = task::spawn(async move {
            let body = inc_counter().await;
            println!("Response from /counter: {}. Endpoint has been called {} times.", body, i);
        });
        handles.push(handle);
    }
    // Await all the spawned tasks
    for handle in handles {
        handle.await.unwrap();
    }
    let body = inc_counter().await;
    println!("Response from /counter at the end: {}", body);
    

    Ok(())
}
