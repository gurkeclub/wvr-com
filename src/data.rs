use message_io::network::NetEvent;
use serde::{Deserialize, Serialize};

use wvr_data::config::project_config::InputConfig;

#[derive(Serialize, Deserialize)]
pub enum ProviderInfo {
    Video((String, String, usize, usize, u32)),
    Picture((String, String, usize, usize)),
    Midi((String, String)),
    Cam((String, String, usize, usize)),
}

#[derive(Serialize, Deserialize)]
pub enum SetInfo {
    BPM(f64),
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Insert((String, InputConfig)),
    Set(SetInfo),
}

pub enum MessageEvent {
    Network(NetEvent<Message>),
}
