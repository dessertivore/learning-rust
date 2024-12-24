# Repo for learning Rust and playing around with it

`src` contains a test API with some basic endpoints.
Current endpoints are:
`/colour` - `GET`
`/addition` - `POST`

## To run the API
```sh
cargo run --bin api
curl localhost:3000/colour
```
This will return e.g. `The best colour is blue!` - a random colour is picked each time.

```sh
curl -X POST localhost:3000/addition -H "Content-Type: application/json" -d '{"nums":"1,2,3"}'
```
This will return `The sum of the numbers is 6` after 2 seconds.

## To call API via Rust
```sh
cargo run --bin client
```
This calls the `/colour` endpoint and then the `/addition` endpoint 20 times asynchronously
in order to test async code in Rust.