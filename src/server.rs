use std::time::Duration;

use anyhow::Result;

use message_io::events::EventReceiver;
use message_io::network::Transport;
use message_io::node::{self, NodeHandler, NodeTask, StoredNetEvent, StoredNodeEvent};

use crate::data::Message;
use wvr_data::config::server_config::ServerConfig;

pub struct OrderServer {
    _handler: NodeHandler<Message>,
    _node_task: NodeTask,
    event_receiver: EventReceiver<StoredNodeEvent<Message>>,
}

impl OrderServer {
    pub fn new(config: &ServerConfig) -> Result<Self> {
        let (handler, listener) = node::split::<Message>();

        let listen_addr = format!("{:}:{:}", config.ip, config.port);
        handler
            .network()
            .listen(Transport::FramedTcp, &listen_addr)?;

        let (node_task, event_receiver) = listener.enqueue();

        Ok(Self {
            _handler: handler,
            _node_task: node_task,
            event_receiver,
        })
    }

    pub fn next_order(&mut self, timeout: Option<Duration>) -> Option<Message> {
        let message = if let Some(timeout) = timeout {
            self.event_receiver.receive_timeout(timeout)?
        } else {
            self.event_receiver.receive()
        };

        match message {
            StoredNodeEvent::Network(event) => match event {
                StoredNetEvent::Message(_, message_data) => {
                    let message: Message = bincode::deserialize(&message_data).unwrap();
                    return Some(message);
                }
                _ => (),
            },
            _ => (),
        }

        None
    }
}
