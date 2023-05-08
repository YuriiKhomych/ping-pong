import asyncio
import logging
import ssl
from aioquic.asyncio import connect, QuicConnectionProtocol
from aioquic.quic.configuration import QuicConfiguration
from aioquic.quic.events import QuicEvent, DatagramFrameReceived, StreamDataReceived

from utils import get_logger


ALPN_WEBTRANSPORT = "webtransport"
# Define the host and port for the server
HOST = "localhost"
PORT = 4433

# Define the path to the server's certificate
SERVER_CERT = "../certificates/localhost.cert"

# Define the ping pong message
PING_MSG = b"ping"

logger = get_logger(__file__)

# Define the handler to process incoming data
class WebTransport(QuicConnectionProtocol):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

    def connection_made(self, transport):
        self._transport = transport
        logger.info("Connected to server")

    def quic_event_received(self, event: QuicEvent):
        if isinstance(event, DatagramFrameReceived):
            message = event.data.decode()
            logger.info(f"Received message from server DatagramFrameReceived: {message}")
            self._quic.send_datagram_frame(PING_MSG)
        elif isinstance(event, StreamDataReceived):
            message = event.data.decode()
            logger.info(f"Received message from server StreamDataReceived: {message}")
            self._quic.send_stream_data(event.stream_id, PING_MSG)

    def connection_lost(self, exc):
        logger.info("Connection closed")


# Define the function to start the ping pong loop
async def start_ping_pong(protocol: WebTransport):
    protocol._quic.send_datagram_frame(PING_MSG)
    protocol.transmit()


async def main():
    # Set up the QUIC configuration
    quic_config = QuicConfiguration(
        alpn_protocols=["webtransport"],
        is_client=True,
        max_datagram_frame_size=65536,
        verify_mode=ssl.CERT_REQUIRED,
        server_name=HOST,
    )
    quic_config.load_verify_locations(SERVER_CERT)

    # Connect to the server
    async with connect(HOST, PORT, configuration=quic_config, create_protocol=WebTransport) as connection:
        asyncio.create_task(start_ping_pong(connection))

        await connection.wait_closed()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("Shutdown client...")
