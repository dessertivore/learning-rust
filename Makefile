.PHONY: api client

api: 
	cargo run --bin api
client: 
	cargo run --bin client