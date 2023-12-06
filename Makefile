# run the server in watch mode
watch:
	cd ./server && \
	echo "server is running on http://localhost:3000" && \
	systemfd --no-pid -- cargo watch -x run
