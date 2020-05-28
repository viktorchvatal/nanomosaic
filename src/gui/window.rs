use gtk::*;
use std::{rc::Rc, cell::RefCell};
use glib::{MainContext};
use super::components::*;
use gdk_pixbuf::{Pixbuf};
use super::{pixbuf::{update_pixbuf, create_pixbuf}};
use crate::{message::{LogicSender, GuiMessage, LogicMessage, Rgba, ImageId}, common::log_err};
use nanocv::{ImgSize, ImgBuf};

pub fn build_ui(app: &Application, path: String, logic: LogicSender) {
    let (gui_tx, gui_rx) = MainContext::channel(glib::PRIORITY_DEFAULT);
    let logic_gui_tx = gui_tx.clone();
    log_err(logic.send(Some(LogicMessage::InitGui(logic_gui_tx))));

    let select_pixbuf = Rc::new(RefCell::new(create_pixbuf(1, 1)));
    let result_pixbuf = Rc::new(RefCell::new(create_pixbuf(1, 1)));

    let splitter = Paned::new(Orientation::Horizontal);
    let window = create_window(app);

    let (select_image, select_box) = create_images(logic.clone(), ImageId::Select);
    let (result_image, result_box) = create_images(logic.clone(), ImageId::Result);

    splitter.pack1(&select_box, false, false);
    splitter.pack2(&result_box, true, true);

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

    splitter.set_position(window.get_size().0/2);

    log_err(logic.send(Some(LogicMessage::LoadImage(path))));
}

fn create_images(logic: LogicSender, id: ImageId) -> (Image, ScrolledWindow) {
    let image = create_image();

    let event_box = EventBox::new();
    event_box.add_events(gdk::EventMask::SCROLL_MASK);
    event_box.add_events(gdk::EventMask::POINTER_MOTION_MASK);
    event_box.add(&image);

    let image_box = create_dummy_scroller();
    image_box.add(&event_box);    
    image_box.set_size_request(50, -1);

    connect_image_resize(logic, image_box.clone(), id);

    (image, image_box)
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
        GuiMessage::Render((id, data)) => match id {
            ImageId::Select => update_image(select_image, select_pixbuf, data),
            ImageId::Result => update_image(result_image, result_pixbuf, data),
        },
    }
}

fn update_image(image: Image, pixbuf: Rc<RefCell<Pixbuf>>, data: ImgBuf<Rgba>) {
    update_pixbuf(&data, pixbuf.clone());
    let inner: &Pixbuf = &pixbuf.borrow();
    image.set_from_pixbuf(Some(inner));
}

fn connect_image_resize(state: LogicSender, scroller: ScrolledWindow, id: ImageId) {
    let c = scroller.clone();
    scroller.connect_size_allocate(move |_widget, _event| {
        on_image_resized(state.clone(), c.clone(), id);
    });
}

fn on_image_resized(state: LogicSender, scroller: ScrolledWindow, id: ImageId) {
    let allocation = scroller.get_allocation();
    let size = ImgSize::new(allocation.width as usize, allocation.height as usize);
    log_err(state.send(Some(LogicMessage::ImageResized((id, size)))));
}