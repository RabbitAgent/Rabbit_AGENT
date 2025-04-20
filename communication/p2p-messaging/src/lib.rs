use libp2p::{
    noise, tcp, yamux, 
    core::upgrade,
    swarm::NetworkBehaviour,
    PeerId, Multiaddr
};

#[derive(NetworkBehaviour)]
pub struct FederatedMessagingBehaviour {
    pub kad: libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>,
    pub floodsub: libp2p::floodsub::Floodsub,
    pub federated_exchange: FederatedParameterExchange,
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "FederatedExchangeEvent")]
pub struct FederatedParameterExchange {
    #[allow(dead_code)]
    keypair: noise::Keypair,
}

impl FederatedParameterExchange {
    pub fn new() -> Self {
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&libp2p::identity::Keypair::Ed25519(
                libp2p::identity::Keypair::generate_ed25519()
            ))
            .unwrap();

        Self { keypair: noise_keys }
    }
}

pub enum FederatedExchangeEvent {
    ParametersReceived {
        peer_id: PeerId,
        model_diff: Vec<u8>,
        signature: Vec<u8>,
    }
}

// p2p-messaging/src/message.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FederatedMessage {
    pub model_parameters: Vec<u8>, // Bincode-serialized tensors
    pub nonce: [u8; 32],
    pub signature: Vec<u8>,
}

impl FederatedMessage {
    pub fn verify(&self, public_key: &[u8]) -> bool {
        use ring::signature::UnparsedPublicKey;
        use ring::signature::Ed25519;
        
        UnparsedPublicKey::new(&Ed25519, public_key)
            .verify(&self.nonce, &self.signature)
            .is_ok()
    }
}
