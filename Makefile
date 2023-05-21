serve-book:
	@cd mdbook && mdbook serve
client:
	@cargo run --quiet --bin rust_client
