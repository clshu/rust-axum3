
pg:
	docker run -d --name pg -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=welcome --restart unless-stopped postgres:15

watch:
	cargo watch -q -c -w crates/   -x "run --bin web-server"

PHONY: pg, watch