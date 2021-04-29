use std::collections::hash_map::HashMap;
use std::time::Duration;

use anyhow::Result;

use message_io::events::EventQueue;
use message_io::network::{Endpoint, NetEvent, NetworkManager};

use crate::data::{Message, MessageEvent};
use wvr_data::config::server_config::ServerConfig;

struct ClientInfo {}

pub struct OrderClient {
    network: NetworkManager,
    endpoint: Endpoint,
    event_queue: EventQueue<MessageEvent>,
}

impl OrderClient {
    pub fn new(config: &ServerConfig) -> Result<Self> {
        let mut event_queue = EventQueue::new();

        let network_sender = event_queue.sender().clone();
        let mut network = NetworkManager::new(move |net_event| {
            network_sender.send(MessageEvent::Network(net_event))
        });

        let listen_addr = format!("{:}:{:}", config.ip, config.port);
        let endpoint = network.connect_tcp(&listen_addr)?;

        Ok(Self {
            network,
            event_queue,
            endpoint,
        })
    }

    pub fn send_order(&mut self, message: Message) -> Result<()> {
        self.network.send(self.endpoint, message)?;
        Ok(())
    }

    pub fn broadcast_state(&mut self) {
        unimplemented!();
    }
}
