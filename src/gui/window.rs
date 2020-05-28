use gtk::*;
use std::{rc::Rc, cell::RefCell};
use glib::{MainContext};
use super::components::*;
use gdk_pixbuf::{Pixbuf};
use super::{pixbuf::{update_pixbuf, create_pixbuf}};
use crate::{message::{LogicSender, GuiMessage, LogicMessage, Rgba}, common::log_err};
use nanocv::ImgBuf;

pub fn build_ui(app: &Application, path: String, logic: LogicSender) {
    let (gui_tx, gui_rx) = MainContext::channel(glib::PRIORITY_DEFAULT);
    let logic_gui_tx = gui_tx.clone();
    log_err(logic.send(Some(LogicMessage::InitGui(logic_gui_tx))));

    let select_pixbuf = Rc::new(RefCell::new(create_pixbuf(1, 1)));
    let result_pixbuf = Rc::new(RefCell::new(create_pixbuf(1, 1)));

    let select_image = create_image();
    let result_image = create_image();

    let splitter = Paned::new(Orientation::Horizontal);
    let window = create_window(app);

    splitter.pack1(&select_image, false, false);
    splitter.pack2(&result_image, true, true);

    window.add(&splitter);
    window.show_all();
    window.maximize();

    let message_logic = logic.clone();
    let message_select_image = select_image.clone();
    let message_result_image = result_image.clone();

    gui_rx.attach(None, move |message: GuiMessage| {
        process_message(
            message_logic.clone(), 
            message, 
            select_pixbuf.clone(),
            result_pixbuf.clone(),
            message_select_image.clone(),
            message_result_image.clone(),
        );

        glib::Continue(true)
    });

    log_err(logic.send(Some(LogicMessage::LoadImage(path))));
}

fn process_message(
    logic: LogicSender, 
    message: GuiMessage, 
    select_pixbuf: Rc<RefCell<Pixbuf>>, 
    result_pixbuf: Rc<RefCell<Pixbuf>>,
    select_image: Image, 
    result_image: Image,
) {
    match message {
        GuiMessage::RenderSelect(image) 
            => update_image(select_image, select_pixbuf, image),
        GuiMessage::RenderResult(image) 
            => update_image(result_image, result_pixbuf, image),
    }
}

fn update_image(image: Image, pixbuf: Rc<RefCell<Pixbuf>>, data: ImgBuf<Rgba>) {
    update_pixbuf(&data, pixbuf.clone());
    let inner: &Pixbuf = &pixbuf.borrow();
    image.set_from_pixbuf(Some(inner));
}

