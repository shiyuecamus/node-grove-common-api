PROTO_DIR = proto
GOLANG_OUT_DIR = go
RUST_OUT_DIR = rust/src

# Install required tools
setup:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	cargo install protoc-gen-prost
	cargo install protoc-gen-prost-serde

# Generate Golang code
generate-golang:
	protoc --proto_path=$(PROTO_DIR) --go_out=$(GOLANG_OUT_DIR) $(PROTO_DIR)/*.proto

# Generate Rust code
generate-rust:
	protoc --proto_path=$(PROTO_DIR) \
	       --prost_out=$(RUST_OUT_DIR) \
	       --prost-serde_out=$(RUST_OUT_DIR) \
	       $(PROTO_DIR)/ng_device.proto $(PROTO_DIR)/ng_driver.proto

generate-all: generate-golang generate-rust

# Clean generated files
clean:
	find $(GOLANG_OUT_DIR) -type f -name '*.pb.go' -delete
	find $(RUST_OUT_DIR) -type f -name 'ng_proto.rs' -delete

help:
	@echo "Available targets:"
	@echo "  setup           - Install required tools"
	@echo "  generate-golang - Generate Golang code from .proto files"
	@echo "  generate-rust   - Generate Rust code from .proto files"
	@echo "  generate-all    - Generate code for both Golang and Rust"
	@echo "  clean           - Remove generated files"