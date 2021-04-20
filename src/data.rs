use message_io::network::NetEvent;
use serde::{Deserialize, Serialize};

use wvr_data::config::project_config::{FilterMode, InputConfig, RenderStageConfig, SampledInput};
use wvr_data::DataHolder;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProviderInfo {
    Video((String, String, usize, usize, u32)),
    Picture((String, String, usize, usize)),
    Midi((String, String)),
    Cam((String, String, usize, usize)),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetInfo {
    Bpm(f64),
    Width(usize),
    Height(usize),
    TargetFps(f64),
    DynamicResolution(bool),
    VSync(bool),
    Screenshot(bool),
    Fullscreen(bool),
    LockedSpeed(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderStageUpdate {
    Filter(String),
    FilterModeParams(FilterMode),
    Variable(String, DataHolder),
    Input(String, SampledInput),
    Name(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Insert((String, InputConfig)),
    Set(SetInfo),
    AddRenderStage(RenderStageConfig),
    RemoveRenderStage(usize),
    UpdateRenderStage(usize, RenderStageUpdate),
    UpdateFinalStage(RenderStageUpdate),
}

pub enum MessageEvent {
    Network(NetEvent<Message>),
}
