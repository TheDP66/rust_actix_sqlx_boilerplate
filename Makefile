dev:
	docker-compose up -d

dev-down:
	docker-compose down

new-migrate:
	sqlx migrate add -r $(name)

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

server:
	cargo watch -q -c -w src/ -x run

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde --features derive
	cargo add serde_json
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"
	cargo install cargo-watch
	cargo install sqlx-cli