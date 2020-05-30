use log::*;
use glib::{Sender as GlibSender};
use std::sync::mpsc::SyncSender;
use nanocv::{ImgSize, ImgBuf};
use crate::common::log_err;
pub type Rgba = [u8; 4];

pub type LogicSender = SyncSender<Option<LogicMessage>>;
pub type CompositorSender = SyncSender<Option<CompositeMessage>>;

#[derive(Clone)]
pub enum LogicMessage {
    InitGui(GlibSender<GuiMessage>),
    LoadImage(String),
    SaveImage(String),
    ImageResized((ImageId, ImgSize)),
    MouseDown((u32, f64, f64)),
    CompositorFinished,
    ReturnBuffer(ImgBuf<Rgba>),
}

#[derive(Clone)]
pub enum CompositeMessage {
    InitGui(GlibSender<GuiMessage>),
    CompositeMosaic((ImgBuf<Rgba>, ImgSize)),
    SaveMosaic((ImgBuf<Rgba>, String)),
}

#[derive(Clone)]
pub enum GuiMessage {
    RenderSource(ImgBuf<Rgba>),
    RenderTarget(ImgBuf<Rgba>),
    RenderLines(SelectionLines),
}

#[derive(Clone, Copy, Debug)]
pub enum ImageId {
    Select,
    Result
}

pub trait MessageReceiver<T>{
    fn receive(&mut self, message: T) -> Result<(), String>;
}

pub fn send<T>(sender: &SyncSender<Option<T>>, message: T) {
    log_err(sender.send(Some(message)));  
}

pub fn send_glib<T>(sender: &Option<GlibSender<T>>, message: T) {
    match sender {
        Some(sender) => log_err(sender.send(message)),
        None => error!("Cannot send message, no sender available."),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SelectionLines {
    pub x1: isize,
    pub x2: isize,
    pub y1: isize,
    pub y2: isize
}
