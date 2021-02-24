
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WsFrame {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub p: f32
}
