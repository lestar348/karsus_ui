# Compatibility Makefile for the new CMake-based lcdhat project.

BUILD_DIR ?= build
INSTALL_PREFIX ?= $(BUILD_DIR)/install
LCDHAT_BACKEND ?= DEV

.PHONY: help configure build all demo install clean reconfigure

help:
	@echo "Targets:"
	@echo "  make configure   - Configure CMake project"
	@echo "  make build       - Build lcdhat library and demo"
	@echo "  make all         - Same as build"
	@echo "  make demo        - Build only lcdhat_basic_demo target"
	@echo "  make install     - Install into INSTALL_PREFIX (default: build/install)"
	@echo "  make clean       - Remove build directory"
	@echo "Variables:"
	@echo "  LCDHAT_BACKEND={DEV|WIRINGPI|BCM2835} (default: DEV)"
	@echo "  BUILD_DIR=<dir>"
	@echo "  INSTALL_PREFIX=<dir>"

configure:
	cmake -S . -B $(BUILD_DIR) -DLCDHAT_BACKEND=$(LCDHAT_BACKEND)

build: configure
	cmake --build $(BUILD_DIR) -j

all: build

demo: configure
	cmake --build $(BUILD_DIR) --target lcdhat_basic_demo -j

install: build
	cmake --install $(BUILD_DIR) --prefix $(INSTALL_PREFIX)

clean:
	rm -rf $(BUILD_DIR)

reconfigure: clean configure
