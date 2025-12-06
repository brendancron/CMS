OPENAPI_SPEC := openapi/api.yaml
GEN_DIR := backend/generated
CLIENT_DIR := client

.PHONY: generate generate-client build run clean

generate:
	@echo "Generating Rust server stubs from OpenAPI..."
	@openapi-generator-cli generate \
		-i $(OPENAPI_SPEC) \
		-g rust-axum \
		-o $(GEN_DIR)

generate-client:
	@echo "Generating TypeScript client from OpenAPI..."
	@openapi-generator-cli generate \
		-i $(OPENAPI_SPEC) \
		-g typescript-axios \
		-o $(CLIENT_DIR) \
		--additional-properties=npmName=cms-api-client,withInterfaces=true

build:
	cd backend && cargo build

run:
	cd backend && cargo run

clean:
	cd backend && cargo clean
	rm -rf $(GEN_DIR)
	rm -rf $(CLIENT_DIR)