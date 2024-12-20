PROTO_DIR = proto
GOLANG_OUT_DIR = gen/go
RUST_OUT_DIR = gen/rust/src

# Install required tools
setup:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	cargo install protoc-gen-prost

# Generate Golang code
generate-golang:
	protoc --proto_path=$(PROTO_DIR) --go_out=$(GOLANG_OUT_DIR) $(PROTO_DIR)/**/*.proto

# Generate Rust code
generate-rust:
	protoc --proto_path=$(PROTO_DIR) --prost_out=$(RUST_OUT_DIR) $(PROTO_DIR)/**/*.proto

# Clean generated files
clean:
	rm -rf $(GOLANG_OUT_DIR)/*
	rm -rf $(RUST_OUT_DIR)/*