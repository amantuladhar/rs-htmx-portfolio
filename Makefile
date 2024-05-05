dev-watch:
	cargo watch -x r
dev:
	cargo run
css:
	pnpx tailwindcss -i ./src/templates/index.css -o ./public/css/styles.css
css-watch:
	pnpx tailwindcss -i ./src/templates/index.css -o ./public/css/styles.css --watch
build-release:css
	cargo build --release