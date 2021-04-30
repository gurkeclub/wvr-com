use anyhow::Result;

use message_io::network::{Endpoint, SendStatus, Transport};
use message_io::node::{self, NodeHandler};

use crate::data::Message;
use wvr_data::config::server_config::ServerConfig;

pub struct OrderClient {
    handler: NodeHandler<Message>,
    server_id: Endpoint,
}

impl OrderClient {
    pub fn new(config: &ServerConfig) -> Result<Self> {
        let (handler, _) = node::split::<Message>();

        let server_addr = format!("{:}:{:}", config.ip, config.port);
        let (server_id, _) = handler
            .network()
            .connect(Transport::FramedTcp, &server_addr)?;
        Ok(Self { handler, server_id })
    }

    pub fn send_order(&mut self, message: Message) -> Result<bool> {
        if let SendStatus::ResourceNotFound = self
            .handler
            .network()
            .send(self.server_id, &bincode::serialize(&message)?)
        {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
