import pytest
from unittest import mock

from .server import WebTransportProtocol, get_protocol_configuration

from aioquic.quic.configuration import QuicConfiguration
from aioquic.quic.events import DatagramFrameReceived, HandshakeCompleted, ConnectionTerminated
from aioquic.quic.connection import QuicConnection

@pytest.fixture
def protocol():
    return WebTransportProtocol(quic=mock.MagicMock(spec=QuicConnection))


def test_get_protocol_configuration():
    configuration = get_protocol_configuration()
    assert isinstance(configuration, QuicConfiguration)
    assert configuration.alpn_protocols == ["webtransport"]
    assert configuration.is_client == False
    assert configuration.max_datagram_frame_size == 65536


def test_quic_event_received_datagram_frame_received(protocol):
    with mock.patch.object(protocol._quic, "send_datagram_frame") as mock_send_datagram_frame:
        protocol.quic_event_received(mock.MagicMock(spec=DatagramFrameReceived, data=b"ping"))
        mock_send_datagram_frame.assert_called_with(b"pong")


def test_quic_event_received_handshake_completed(protocol):
    protocol.quic_event_received(mock.MagicMock(spec=HandshakeCompleted))
    assert True


def test_quic_event_received_connection_terminated(protocol):
    protocol.quic_event_received(mock.MagicMock(spec=ConnectionTerminated))
    assert True
