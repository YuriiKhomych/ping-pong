# Ping pong python client and server

This is a simple WebTransport application that sends a message to the server and waits for a response. The server echoes the message back to the client.

Before running the application, you need to generate certificates for the server. You can do this by running the command provided in the main README.md file.

To run the server you need to run the following command in the `server` directory of the repository:

```
poetry shell && poetry install && python server.py
```

To run python client you need to run the following command in the `server` directory of the repository:

```
poetry shell && poetry install && python client.py
```

To run tests you need to run the following command in the `server` directory of the repository:
```
poetry shell && poetry install && poetry run python -m pytest
```
