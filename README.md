WebTransport Ping-Pong App

This is a simple WebTransport application that sends `ping` message to the server and waits for a response. The server echoes with `pong` message back to the client.
Client is written in Rust and server is written in Python.

Before running the application, you need to generate certificates for the server. You can do this by running the following command in the root directory of the repository:

```
cd certificates && openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -out localhost.cert -keyout localhost.key -subj '/CN=localhost' -config openssl.cfg
```

To run RUST client you need to run the following command in the root directory of the repository:

```
cd client && cargo run
```

To run the server you need to run the following command in the root directory of the repository:

```
cd poetry shell && poetry install && python server.py
```

To run tests for server you need to run the following command in the root directory of the repository:
```
cd server && poetry shell && poetry run python -m pytest
```
