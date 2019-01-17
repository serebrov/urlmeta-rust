build:
	docker build -t urlmeta-rust .

run: build
	docker run -p 8000:8000 --rm --name urlmeta-rust-app urlmeta-rust urlmeta
