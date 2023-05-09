WebTransport Ping-Pong App

Essential Requirements:

●	Preferably the Client should be written in RUST, but feel free to use any other language you are comfortable with or believe is best for this operation. ✅

●	The Server should be written in Python ✅

●	Communication protocol should be in WebTransport ✅

●	Provide some unit test coverage for both sides to demonstrate your skill in unit testing ✅

Desirable Requirements:

●	Make the communication channel secure or suggest what security measures you would implement given more time. ✅👨🏻‍💻

For this app we're using TLS 1.3 with self-signed certificates which is okay for local development but not for production as it cannot prevent MITM attacks. For production we would use a certificate signed by a trusted CA or at least Let's Encrypt.
Additional ideas for security:
1. Use strong encryption algorithms to encrypt the data exchanged between client and server.
2. Implement secure authentication mechanisms such as multi-factor authentication or two-factor authentication to ensure that only authorized users can access the system. 
3. Implement firewalls to restrict access to the communication channel and to protect against attacks such as DDoS attacks.
4. Implement intrusion detection and prevention systems to monitor the communication channel for unusual activity and to prevent attacks.
5. Keep the software and systems up to date with the latest security patches and updates to address known vulnerabilities.
6. Regularly audit and monitor the system for security breaches and vulnerabilities, and take appropriate measures to address them.

●	Provide a plan for Kubernetes deployment 👨🏻‍💻
1. Create a Docker image for the server.
2. Create a Docker image for the client.
3. Create a Kubernetes cluster on cloud provider.
4. Create a Kubernetes deployment YAML file for the Rust client and Python server.
5. Apply the deployment YAML files to the Kubernetes cluster.
6. Configure networking between the client and server.
7. Configure TLS certificates for the client and server.
8. Add a load balancer to distribute the load between multiple instances of the server.
9. Verify that the client can connect to the server and send/receive data.

●	Provide a plan/design for an auto-recovery mechanism for both sides (in case of a temporary connection failure). Feel free to implement that if you have enough time. 👨🏻‍💻
1. Implement a retry mechanism to retry the connection if it fails. To implement that we can use the exponential backoff algorithm to increase the time between retries. In python it's backoff library and in rust we can use tokio-retry.
2. Implement a timeout mechanism to cancel the connection attempt if it takes too long.
3. Implement a backoff mechanism to increase the time between retries if the connection fails multiple times in a row.
4. Implement a circuit breaker mechanism to stop retrying if the connection fails too many times in a row.
5. Implement a health check mechanism to check the health of the server and stop retrying if the server is down.
6. Implement a load balancing mechanism to distribute the load between multiple instances of the server.


●	Provide integration tests 👨🏻‍💻
Integration tests use cases:
1. Test that the client can connect to the server and send/receive data. (Partially implemented on the Rust client side but only with Rust server)
2. Test that the client can connect to the server and send/receive data. (Not implemented on the Python server side)
3. Test the handling of errors or unexpected input from the client or the server.
4. Test the handling of timeouts and retries.
5. Test the handling of multiple concurrent connections and requests.
6. Test the reliability of the system by simulating network failures or other disruptions and ensuring that the auto-recovery mechanism works as expected.


●	Can you think of a way for the client to auto-discover the server without the need to point it to the exact server endpoint? 👨🏻‍💻

We can use DNS to map a domain name to the IP address of the server. The client/server can then query the DNS server to resolve the domain name to the IP address. This approach can be used with tools such as Consul, etcd, or Kubernetes DNS.

Also we may use similar approach like service registry to maintain a list of available servers and their endpoints. The client can query the registry to discover available servers. This approach is commonly used in microservices architecture.

So in general we should use some intermediary to discover the endpoint that is not hardcoded in our app. This will allow us to change the endpoint without changing the code.


# WebTransport Ping-Pong App
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
