.PHONY: api client euler

api: 
	cargo run --bin api
client: 
	cargo run --bin client
euler: 
	cargo run --bin project-euler