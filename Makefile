# Makefile for XOR Crusher LV2 Plugin

# The name of the LV2 bundle directory.
BUNDLE_NAME = xor_crusher.lv2
TARGET_DIR ?= target
BUILD_PROFILE ?= release
LIB_NAME = libxor_crusher.so
SO_NAME = xor_crusher.so
LV2_DEST ?= $(HOME)/.lv2

# Default target
all: bundle

# Build the Rust shared library.
build:
	@echo "Building Rust library..."
	@cargo build --$(BUILD_PROFILE)

# Create the .lv2 bundle directory structure.
bundle: build
	@echo "Creating LV2 bundle..."
	@mkdir -p $(TARGET_DIR)/$(BUNDLE_NAME)
	@cp $(TARGET_DIR)/$(BUILD_PROFILE)/$(LIB_NAME) $(TARGET_DIR)/$(BUNDLE_NAME)/$(SO_NAME)
	@cp data/* $(TARGET_DIR)/$(BUNDLE_NAME)/
	@echo "Bundle created at $(TARGET_DIR)/$(BUNDLE_NAME)"

# Install the bundle to the user's LV2 directory.
install: bundle
	@echo "Installing to $(LV2_DEST)..."
	@mkdir -p $(LV2_DEST)
	@cp -r $(TARGET_DIR)/$(BUNDLE_NAME) $(LV2_DEST)/
	@echo "Installation complete."

# Run linters
check:
	@echo "Running linters..."
	@cargo check
	@cargo clippy -- -D warnings

# Remove build artifacts.
clean:
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf $(TARGET_DIR)

# Create a zip file of the bundle for distribution.
zip: bundle
	@echo "Creating zip archive..."
	@cd $(TARGET_DIR) && zip -r ../xor_crusher.lv2.zip $(BUNDLE_NAME)
	@echo "Zip file created at xor_crusher.lv2.zip"

.PHONY: all build bundle install check clean zip
