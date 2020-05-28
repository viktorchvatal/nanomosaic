use glib::{Sender as GlibSender};
use std::sync::mpsc::SyncSender;
use nanocv::ImgBuf;
pub type Rgba = [u8; 4];

pub type LogicSender = SyncSender<Option<LogicMessage>>;

#[derive(Clone, Debug)]
pub enum LogicMessage {
    InitGui(GlibSender<GuiMessage>),
    LoadImage(String),
}

#[derive(Clone, Debug)]
pub enum GuiMessage {
    RenderSelect(ImgBuf<Rgba>),
    RenderResult(ImgBuf<Rgba>),
}