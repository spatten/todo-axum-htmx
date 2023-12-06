# run the server in watch mode
watch:
	echo "server is running on http://localhost:3000" && \
	systemfd --no-pid -s http::3000 -- cargo watch -x run
