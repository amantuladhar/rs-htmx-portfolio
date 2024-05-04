dev-watch:
	cargo watch -x r
dev:
	cargo run
css:
	pnpx tailwindcss -i ./src/templates/index.css -o ./public/css/output.css
css-watch:
	pnpx tailwindcss -i ./src/templates/index.css -o ./public/css/output.css --watch