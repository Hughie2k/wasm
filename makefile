run: src/lib.rs
	webpack-cli build
	cd dist
	py -m http.server 8080
	