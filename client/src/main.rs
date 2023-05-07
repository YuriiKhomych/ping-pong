use std::{
    error::Error,
    fs::File,
    io::BufReader,
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};
use tokio;

use quinn::{ClientConfig, Connection, Endpoint, ServerConfig};
use rustls::{Certificate, ClientConfig as RustlsClientConfig, RootCertStore};
use std::str;

const MSG: &[u8; 4] = b"ping";
const ALPN_WEBTRANSPORT: &[&[u8]] = &[b"webtransport"];
const SERVER_HOST: &str = "localhost";
const SERVER_PORT: u16 = 4433;
const CERTIFICATE_PATH: &str = "../certificates/localhost.cert";

struct WebTransportClient {
    endpoint: Endpoint,
    server_address: SocketAddr,
}

impl WebTransportClient {
    // Connect to the server
    async fn get_connection(&self) -> Result<Connection, Box<dyn Error>> {
        let connection: Connection = self
            .endpoint
            .connect(self.server_address, SERVER_HOST)?
            .await?;
        println!("[client] connected: addr={}", connection.remote_address());
        Ok(connection)
    }

    // Send a ping request to the server
    async fn send_request(&self, connection: Connection) -> Result<(), Box<dyn Error>> {
        match connection.send_datagram(MSG[..].into()) {
            Ok(_) => println!("[client] sent request {:?}", str::from_utf8(MSG).unwrap()),
            Err(e) => {
                eprintln!("Failed to send datagram: {}", e);
                return Err(Box::new(e));
            }
        }
        Ok(())
    }

    // Receive a response from the server
    async fn receive_response(&self, connection: Connection) -> Result<String, Box<dyn Error>> {
        let response = match connection.read_datagram().await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to read datagram: {}", e);
                return Err(Box::new(e));
            }
        };
        let response_message: &str = match str::from_utf8(&response) {
            Ok(message) => message,
            Err(e) => {
                eprintln!("Failed to decode response message: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(response_message.to_owned())
    }

    // Send a ping request and receive a pong response
    async fn ping_pong(&self, connection: Connection) -> Result<String, Box<dyn Error>> {
        self.send_request(connection.clone()).await?;
        let response_message: String = self.receive_response(connection.clone()).await?;
        Ok(response_message)
    }

    // Run the client forever
    async fn run_forever(&self) -> Result<(), Box<dyn Error>> {
        let connection: Connection = self.get_connection().await?;

        loop {
            let response_message: String = self.ping_pong(connection.clone()).await?;
            println!("[client] got response: {:?}", response_message);
        }
    }
}

// Read the server's certificate
fn read_server_certs() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut cert_chain_reader: BufReader<File> = BufReader::new(File::open(CERTIFICATE_PATH)?);
    let server_certs: Vec<Vec<u8>> = rustls_pemfile::certs(&mut cert_chain_reader)?;

    Ok(server_certs)
}

// Create a root certificate store from the server's certificate
fn get_root_cert_store(server_certs: Vec<Vec<u8>>) -> Result<RootCertStore, Box<dyn Error>> {
    let mut root_cert_store: RootCertStore = RootCertStore::empty();
    for cert in server_certs {
        root_cert_store.add(&Certificate(cert))?;
    }

    Ok(root_cert_store)
}

// Configure the client to use the server's certificate and ALPN WebTransport protocol
fn configure_client(server_certs: RootCertStore) -> Result<ClientConfig, Box<dyn Error>> {
    let mut client_crypto: rustls::ClientConfig = RustlsClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(server_certs)
        .with_no_client_auth();

    client_crypto.alpn_protocols = ALPN_WEBTRANSPORT.iter().map(|&x| x.into()).collect();
    let client_config: ClientConfig = ClientConfig::new(Arc::new(client_crypto));

    Ok(client_config)
}

// Create a QUIC endpoint for the client
fn make_client_endpoint(certs: RootCertStore) -> Result<Endpoint, Box<dyn Error>> {
    // Configure the client to use the server's certificate and ALPN WebTransport protocol
    let client_config: ClientConfig = configure_client(certs)?;
    let client_address: SocketAddr = format!("{}:0", SERVER_HOST)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    // Create a QUIC endpoint for the client
    let mut endpoint: Endpoint = Endpoint::client(client_address)?;
    endpoint.set_default_client_config(client_config);

    Ok(endpoint)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // todo move to lazy_static
    let server_address: SocketAddr = format!("{}:{}", SERVER_HOST, SERVER_PORT)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    // Read the server's certificate
    let server_certs: Vec<Vec<u8>> = read_server_certs()?;
    let certs: RootCertStore = get_root_cert_store(server_certs)?;

    let endpoint: Endpoint = make_client_endpoint(certs)?;

    let client: WebTransportClient = WebTransportClient {
        endpoint,
        server_address,
    };
    println!("Client starting...");
    client.run_forever().await?;
    Ok(())
}

#[tokio::test]
async fn test_webtransport_client() {
    // todo move to lazy_static
    let server_address: SocketAddr = format!("{}:{}", SERVER_HOST, SERVER_PORT)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    // Create certificate and key
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let key = rustls::PrivateKey(cert.serialize_private_key_der());
    let cert = rustls::Certificate(cert.serialize_der().unwrap());
    let mut roots = rustls::RootCertStore::empty();
    roots.add(&cert).unwrap();

    // setup server config
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .unwrap();
    server_crypto.max_early_data_size = u32::MAX;
    server_crypto.alpn_protocols = ALPN_WEBTRANSPORT.iter().map(|&x| x.into()).collect();
    let server_config = ServerConfig::with_crypto(Arc::new(server_crypto));

    // setup server
    let server_endpoint = Endpoint::server(server_config, server_address).unwrap();

    // setup client
    let endpoint = make_client_endpoint(roots).unwrap();
    let client: WebTransportClient = WebTransportClient {
        endpoint,
        server_address,
    };

    // run client and server
    let (client_res, server_res) = tokio::join!(client.get_connection(), async {
        server_endpoint.accept().await.unwrap().await
    });
    let client_connection = client_res.unwrap();
    let server_connection = server_res.unwrap();

    // send message from client to server
    let done = tokio::sync::Notify::new();
    let (client_response, server_response) = tokio::join!(
        async {
            let client_response: String = client.ping_pong(client_connection).await.unwrap();
            done.notify_waiters();
            client_response
        },
        async {
            server_connection.send_datagram(b"pong"[..].into()).unwrap();
            done.notified().await;
            let server_response = server_connection.read_datagram().await.unwrap();
            server_response
        }
    );

    // check that the message was received
    assert!(*client_response == *"pong".to_owned());
    assert!(*server_response == *MSG);
}
