
build-openapi:
	docker run --rm \
		-v ${PWD}:/local openapitools/openapi-generator-cli generate \
		-i /local/openapi/starwars.yaml \
		-g rust \
		-o /local/generated \
		--additional-properties packageName=starwars-api

build-python:
	cargo build --release
	cargo run --bin uniffi-bindgen generate --library target/release/libstarwars.dylib --language python --out-dir sdk/python
	cp -R target/release/libstarwars.dylib sdk/python/

clean:
	cargo clean
	rm -r sdk/python/libstarwars.dylib