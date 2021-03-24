install:
	brew install gcc-arm-embedded minicom openocd arm-none-eabi-gcc
	rustup target add thumbv6m-none-eabi

serve:
	openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg

build:
	cargo build --target thumbv6m-none-eabi

deploy: build
	arm-none-eabi-gdb -q target/thumbv6m-none-eabi/debug/sos
