use glib::{Sender as GlibSender};
use std::sync::mpsc::SyncSender;
use nanocv::{ImgSize, ImgBuf};
pub type Rgba = [u8; 4];

pub type LogicSender = SyncSender<Option<LogicMessage>>;

#[derive(Clone)]
pub enum LogicMessage {
    InitGui(GlibSender<GuiMessage>),
    LoadImage(String),
    ImageResized((ImageId, ImgSize)),
}

#[derive(Clone)]
pub enum GuiMessage {
    Render((ImageId, ImgBuf<Rgba>)),
}

#[derive(Clone, Copy, Debug)]
pub enum ImageId {
    Select,
    Result
}