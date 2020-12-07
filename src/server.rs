use std::collections::hash_map::HashMap;
use std::time::Duration;

use message_io::events::EventQueue;
use message_io::network::{Endpoint, NetEvent, NetworkManager};

use crate::data::{Message, MessageEvent};
use wvr_data::config::server_config::ServerConfig;

struct ClientInfo {}

pub struct OrderServer {
    clients: HashMap<Endpoint, ClientInfo>,
    _network: NetworkManager,
    event_queue: EventQueue<MessageEvent>,
}

impl OrderServer {
    pub fn new(config: &ServerConfig) -> Self {
        let mut event_queue = EventQueue::new();

        let network_sender = event_queue.sender().clone();
        let mut network = NetworkManager::new(move |net_event| {
            network_sender.send(MessageEvent::Network(net_event))
        });

        let clients: HashMap<Endpoint, ClientInfo> = HashMap::new();

        let listen_addr = format!("{:}:{:}", config.ip, config.port);
        match network.listen_tcp(&listen_addr) {
            Ok(_) => println!("TCP Server running at {}", listen_addr),
            Err(_) => panic!("Can not listening at selected interface/port"),
        }

        Self {
            clients,
            _network: network,
            event_queue,
        }
    }

    pub fn next_order(&mut self, timeout: Option<Duration>) -> Option<Message> {
        let message = if let Some(timeout) = timeout {
            self.event_queue.receive_event_timeout(timeout)?
        } else {
            self.event_queue.receive()
        };

        match message {
            MessageEvent::Network(net_event) => match net_event {
                NetEvent::Message(_endpoint, message) => {
                    return Some(message);
                }
                NetEvent::AddedEndpoint(endpoint) => {
                    self.clients.insert(endpoint, ClientInfo {});
                    println!(
                        "Client ({}) connected (total clients: {})",
                        endpoint.addr(),
                        self.clients.len()
                    );
                }
                NetEvent::RemovedEndpoint(endpoint) => {
                    self.clients.remove(&endpoint).unwrap();
                    println!(
                        "Client ({}) disconnected (total clients: {})",
                        endpoint.addr(),
                        self.clients.len()
                    );
                }
            },
        }

        None
    }

    pub fn broadcast_state(&mut self) {
        unimplemented!();
    }
}
