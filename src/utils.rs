#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Reserved {
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
    pub top: u32,
}

#[derive(Default)]
pub struct ScreenInfo {
    pub id: u8,
    pub width: u32,
    pub height: u32,
}