RUSTC_OPTION = --target wasm32-unknown-unknown --crate-type=cdylib

all: sum.wasm
	mv ./*.wasm ./public/js/

sum.wasm: src/sum.rs
	rustc $(RUSTC_OPTION) -O $<
