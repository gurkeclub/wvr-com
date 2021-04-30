use serde::{Deserialize, Serialize};

use wvr_data::config::project_config::{
    Automation, BufferPrecision, FilterMode, InputConfig, RenderStageConfig, SampledInput, Speed,
};
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
    VariableAutomation(String, Automation),
    Input(String, SampledInput),
    Precision(BufferPrecision),
    Name(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputUpdate {
    SetWidth(usize),
    SetHeight(usize),
    SetPath(String),
    SetSpeed(Speed),
}

#[derive(Debug, Serialize, Deserialize)]
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
