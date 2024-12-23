# Repo for learning Rust and playing around with it

`src` contains a test API with some basic endpoints.
Current endpoints are:
`/colour` - `GET`

## To run the API
```sh
cargo run
curl localhost:3000/colour
```
This will return e.g. `The best colour is blue!` - a random colour is picked each time.