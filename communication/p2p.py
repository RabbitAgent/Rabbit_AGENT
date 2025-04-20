# File: rabbitagent/communication/p2p.py

import asyncio
import json
from typing import Callable, Dict, List, Tuple

from rabbitagent.utils.logger import get_logger

logger = get_logger("communication.p2p")


class PeerConnection:
    def __init__(
        self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter, addr: Tuple[str, int]
    ):
        self.reader = reader
        self.writer = writer
        self.address = addr
        self.id = f"{addr[0]}:{addr[1]}"

    async def send(self, message: Dict) -> None:
        """Send a JSON‑serializable message to this peer."""
        data = json.dumps(message).encode() + b"\n"
        self.writer.write(data)
        await self.writer.drain()

    async def close(self) -> None:
        """Gracefully close this peer connection."""
        self.writer.close()
        await self.writer.wait_closed()


class P2PNode:
    def __init__(
        self,
        host: str,
        port: int,
        bootstrap_peers: List[Tuple[str, int]],
        message_handler: Callable[[str, Dict], None],
    ):
        """
        :param host: local bind address
        :param port: local bind port
        :param bootstrap_peers: list of (host, port) to dial at startup
        :param message_handler: callback(peer_id, message_dict)
        """
        self.host = host
        self.port = port
        self.bootstrap_peers = bootstrap_peers
        self.handler = message_handler
        self.server: asyncio.base_events.Server = None  # type: ignore
        self.peers: Dict[str, PeerConnection] = {}

    async def start_server(self) -> None:
        """Start listening for incoming peer connections."""
        self.server = await asyncio.start_server(self._accept_connection, self.host, self.port)
        logger.info(f"P2P server listening on {self.host}:{self.port}")

    async def _accept_connection(
        self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter
    ) -> None:
        addr = writer.get_extra_info("peername")
        peer = PeerConnection(reader, writer, addr)
        self.peers[peer.id] = peer
        logger.info(f"Accepted connection from {peer.id}")
        await self._read_loop(peer)

    async def connect_to_bootstrap(self) -> None:
        """Dial out to bootstrap peers."""
        for host, port in self.bootstrap_peers:
            try:
                reader, writer = await asyncio.open_connection(host, port)
                peer = PeerConnection(reader, writer, (host, port))
                self.peers[peer.id] = peer
                logger.info(f"Connected to bootstrap peer {peer.id}")
                asyncio.create_task(self._read_loop(peer))
            except Exception as e:
                logger.error(f"Failed to connect to {host}:{port} — {e}")

    async def _read_loop(self, peer: PeerConnection) -> None:
        """Read incoming messages from a peer."""
        while True:
            try:
                line = await peer.reader.readline()
                if not line:
                    break
                msg = json.loads(line.decode())
                logger.debug(f"Received from {peer.id}: {msg}")
                self.handler(peer.id, msg)
            except Exception as e:
                logger.error(f"Error reading from {peer.id}: {e}")
                break
        await peer.close()
        self.peers.pop(peer.id, None)
        logger.info(f"Closed connection: {peer.id}")

    async def broadcast(self, message: Dict) -> None:
        """Send a message to all connected peers."""
        for peer in list(self.peers.values()):
            try:
                await peer.send(message)
            except Exception as e:
                logger.error(f"Broadcast to {peer.id} failed: {e}")

    async def send_to(self, peer_id: str, message: Dict) -> None:
        """Send a message to a specific peer."""
        peer = self.peers.get(peer_id)
        if not peer:
            logger.warning(f"Peer {peer_id} not found")
            return
        await peer.send(message)

    async def shutdown(self) -> None:
        """Close all connections and stop the server."""
        for peer in list(self.peers.values()):
            await peer.close()
        if self.server:
            self.server.close()
            await self.server.wait_closed()
        logger.info("P2P node shut down")


# Example standalone usage
if __name__ == "__main__":

    async def on_message(peer_id: str, msg: Dict):
        print(f"[{peer_id}] → {msg}")

    async def main():
        node = P2PNode(
            host="127.0.0.1",
            port=9000,
            bootstrap_peers=[("127.0.0.1", 9001)],
            message_handler=on_message,
        )
        await node.start_server()
        await node.connect_to_bootstrap()

        # send a greeting after peers connect
        await asyncio.sleep(1)
        await node.broadcast({"type": "greeting", "payload": "hello peers"})
        await asyncio.Event().wait()  # keep running

    asyncio.run(main())
