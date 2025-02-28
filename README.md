# Repo for learning Rust and playing around with it

`src` contains a test API with some basic endpoints.
Current endpoints are:
`/colour` - `GET`
`/addition` - `POST`

## To run the API
```sh
make api
curl localhost:3000/colour
```
This will return e.g. `The best colour is blue!` - a random colour is picked each time.

```sh
curl -X POST localhost:3000/addition -H "Content-Type: application/json" -d '{"nums":"1,2,3"}'
```
This will return `The sum of the numbers is 6` after 2 seconds.

## To call API via Rust
```sh
make client
```
This calls the `/colour` endpoint then adds to the counter 1000 times with the `/counter`
POST endpoint. I was trying to elicit race conditions but have failed at that 
unfortunately. Lastly, it calls `/counter` with a GET request to get the final count.

# To run Project Euler problems
```sh
make euler 1 1000
```
This runs project Euler puzzle 1 with the input 1000. Change args as desired.
Results will print to console.