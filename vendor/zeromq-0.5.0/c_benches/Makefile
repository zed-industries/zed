all: client server

client: client.c
	clang -L/usr/lib/x86_64-linux-gnu -lzmq client.c -o client


server: server.c
	clang -L/usr/lib/x86_64-linux-gnu -lzmq server.c -o server
