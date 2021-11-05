use serde::{Deserialize, Serialize};

use wvr_data::config::filter::FilterMode;
use wvr_data::config::input::InputConfig;
use wvr_data::config::rendering::RenderStageConfig;
use wvr_data::types::DataHolder;
use wvr_data::types::{Automation, BufferPrecision, InputSampler, Speed};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProviderInfo {
    Video((String, String, usize, usize, u32)),
    Picture((String, String, usize, usize)),
    Midi((String, String)),
    Cam((String, String, usize, usize)),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RenderStageUpdate {
    Filter(String),
    FilterModeParams(FilterMode),
    Variable(String, DataHolder),
    VariableAutomation(String, Automation),
    VariableOffset(String, Option<(String, DataHolder)>),
    Input(String, InputSampler),
    Precision(BufferPrecision),
    Name(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InputUpdate {
    SetWidth(usize),
    SetHeight(usize),
    SetPath(String),
    SetSpeed(Speed),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Start,
    Stop,
    Pause,
    Insert((String, InputConfig)),
    Set(SetInfo),

    AddInput(String, InputConfig),
    RemoveInput(String),
    RenameInput(String, String),
    UpdateInput(String, InputUpdate),

    AddRenderStage(RenderStageConfig),
    RemoveRenderStage(usize),
    MoveRenderStage(usize, usize),
    UpdateRenderStage(usize, RenderStageUpdate),
    UpdateFinalStage(RenderStageUpdate),
}
