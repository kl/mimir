# run the server in development mode (enables autoreload)
dev-server:
	cargo watch -x run --watch web/templates --features=dev-server | grep -v "/health_check" | bunyan

# (re)-creates the dev sqlite database
init-db:
	target/debug/mimir-init-db 12345678
	
