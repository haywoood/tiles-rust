dev:
	cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target web && rollup ./main.js --format iife --file ./pkg/bundle.js"
