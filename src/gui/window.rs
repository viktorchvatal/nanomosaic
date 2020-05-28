use gtk::*;
use std::{rc::Rc, cell::RefCell};
use glib::{MainContext};
use super::components::*;
use super::{pixbuf::create_pixbuf};
use crate::message::GuiMessage;

pub fn build_ui(app: &Application) {
    let (gui_tx, gui_rx) = MainContext::channel(glib::PRIORITY_DEFAULT);

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

    gui_rx.attach(None, move |message: GuiMessage| {
        glib::Continue(true)
    });
}

