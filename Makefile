# DEV
dev-watch:
	cargo watch -x r
dev:
	cargo run
css:
	pnpx tailwindcss -i ./src/templates/index.css -o ./public/css/styles.css
css-watch:
	pnpx tailwindcss -i ./src/templates/index.css -o ./public/css/styles.css --watch

# Build
build-release:css
	cargo build --release

# DB
db-install-migration-cli:
	cargo install sqlx-cli --no-default-features --features native-tls,postgres
db-add-migration:
	sqlx migrate add -r $$name
db-run-migration:
	sqlx migrate run
db-revert-migration:
	sqlx migrate revert
db-offline:
	cargo sqlx prepare