import time

from aioquic.asyncio import QuicConnectionProtocol, serve

import asyncio
import pathlib

from aioquic.quic.configuration import QuicConfiguration
from aioquic.quic.events import DatagramFrameReceived, StreamDataReceived, HandshakeCompleted, ConnectionTerminated, QuicEvent
from aioquic.quic.logger import QuicLogger
from utils import get_logger

ROOT = pathlib.Path(__file__).parent
# todo use dotenv
CERTIFICATE_PATH = ROOT / "../certificates/localhost.cert"
KEY_PATH = ROOT / "../certificates/localhost.key"

HOST = "localhost"
PORT = 4433

logger = get_logger(__file__)


class WebTransportProtocol(QuicConnectionProtocol):
    def quic_event_received(self, event: QuicEvent):
        # todo use super and test it
        if isinstance(event, DatagramFrameReceived):
            message = event.data.decode()
            logger.info(f"Received message from client DatagramFrameReceived: {message}")
            time.sleep(1)
            self._quic.send_datagram_frame(b"pong")
        elif isinstance(event, HandshakeCompleted):
            logger.info("Handshake completed")
        elif isinstance(event, ConnectionTerminated):
            logger.info("Connection terminated")


def get_protocol_configuration():
    configuration = QuicConfiguration(
        alpn_protocols=["webtransport"],
        is_client=False,
        max_datagram_frame_size=65536,
        quic_logger=QuicLogger(),
    )
    configuration.load_cert_chain(CERTIFICATE_PATH, KEY_PATH)
    return configuration


async def main():
    logger.info("Starting ping pong webtransport server...")

    await serve(
        HOST,
        PORT,
        configuration=get_protocol_configuration(),
        create_protocol=WebTransportProtocol,
    )
    logger.info("Server started. Waiting for connections...")
    await asyncio.Future()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("Shutdown server...")

