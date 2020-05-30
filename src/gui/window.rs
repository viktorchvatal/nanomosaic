use gtk::*;
use std::{rc::Rc, cell::RefCell};
use glib::{MainContext};
use super::components::*;
use gdk_pixbuf::{Pixbuf};
use super::{
    file_dialogs::{save_file_dialog, open_file_dialog}, 
    pixbuf::{update_pixbuf, create_pixbuf, horizontal_line, vertical_line}
};
use crate::{common::log_err, message::*};
use nanocv::{ImgSize, ImgBuf};
use gdk::EventButton;

pub fn build_ui(
    app: &Application, 
    path: Option<String>, 
    logic: LogicSender,
    composite: CompositorSender,
) {
    let (gui_tx, gui_rx) = MainContext::channel(glib::PRIORITY_DEFAULT);
    let logic_gui_tx = gui_tx.clone();
    let composite_gui_tx = gui_tx.clone();
    
    send(&logic, LogicMessage::InitGui(logic_gui_tx));
    send(&composite, CompositeMessage::InitGui(composite_gui_tx));

    let select_pixbuf = Rc::new(RefCell::new(create_pixbuf(1, 1)));
    let result_pixbuf = Rc::new(RefCell::new(create_pixbuf(1, 1)));

    let splitter = Paned::new(Orientation::Horizontal);
    let window = create_window(app);

    let (select_image, select_events, select_box) = create_images(logic.clone(), ImageId::Select);
    let (result_image, _, result_box) = create_images(logic.clone(), ImageId::Result);

    connect_image_mouse_down(select_events.clone(), logic.clone());
    connect_image_mouse_move(select_events.clone(), logic.clone());

    splitter.pack1(&select_box, false, false);
    splitter.pack2(&result_box, true, true);

    let load_button = create_load_button(logic.clone(), window.clone());
    let save_button = create_save_button(logic.clone(), window.clone());

    let top_panel = Box::new(Orientation::Horizontal, 0);
    top_panel.pack_start(&load_button, false, false, 5);
    top_panel.pack_start(&save_button, false, false, 5);

    let main_panel = Box::new(Orientation::Vertical, 0);
    main_panel.pack_start(&top_panel, false, false, 5);
    main_panel.pack_start(&splitter, true, true, 5);

    window.add(&main_panel);
    window.show_all();
    window.maximize();

    let message_logic_sender = logic.clone();
    let message_select_image = select_image.clone();
    let message_result_image = result_image.clone();

    gui_rx.attach(None, move |message: GuiMessage| {
        process_message(
            message_logic_sender.clone(),
            message, 
            select_pixbuf.clone(),
            result_pixbuf.clone(),
            message_select_image.clone(),
            message_result_image.clone(),
        );

        glib::Continue(true)
    });

    splitter.set_position(window.get_size().0/2);

    if let Some(path) = path {
        send(&logic, LogicMessage::LoadImage(path));
    }
}

fn create_load_button(logic: LogicSender, window: ApplicationWindow) -> Button {
    let button = Button::new();
    button.add(&Label::new("Load image"));
    button.connect_clicked(move |_| {
        if let Some(path) = open_file_dialog(window.clone()) {
            send(&logic, LogicMessage::LoadImage(path));
        }
    });   
    button    
}

fn create_save_button(logic: LogicSender, window: ApplicationWindow) -> Button {
    let button = Button::new();
    button.add(&Label::new("Save image"));
    button.connect_clicked(move |_| {
        if let Some(path) = save_file_dialog(window.clone()) {
            send(&logic, LogicMessage::SaveImage(path));
        }
    });   
    button    
}

fn create_images(logic: LogicSender, id: ImageId) -> (Image, EventBox, ScrolledWindow) {
    let image = create_image();

    let event_box = EventBox::new();
    event_box.add_events(gdk::EventMask::SCROLL_MASK);
    event_box.add_events(gdk::EventMask::POINTER_MOTION_MASK);
    event_box.add(&image);

    let image_box = create_dummy_scroller();
    image_box.add(&event_box);    
    image_box.set_size_request(50, -1);

    connect_image_resize(logic, image_box.clone(), id);

    (image, event_box, image_box)
}

fn connect_image_mouse_down(image: EventBox, logic: LogicSender) {
    image.connect_button_press_event(move |_image, event: &EventButton| {
        let (x, y) = event.get_position();
        send(&logic, LogicMessage::MouseDown((event.get_button(), x, y)));
        Inhibit(true)
    });
}

fn connect_image_mouse_move(image: EventBox, logic: LogicSender) {
    image.connect_motion_notify_event(move |_image, event| {
        let (x, y) = event.get_position();

        if event.get_state().contains(gdk::ModifierType::BUTTON1_MASK) {
            send(&logic, LogicMessage::MouseDown((1, x, y)));
        }

        if event.get_state().contains(gdk::ModifierType::BUTTON3_MASK) {
            send(&logic, LogicMessage::MouseDown((3, x, y)));
        }

        Inhibit(true)
    });
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
        GuiMessage::RenderSource(image) => {
            update_image(select_image, select_pixbuf, &image);
            send(&logic, LogicMessage::ReturnBuffer(image));
        }
        GuiMessage::RenderTarget(data) => {
            update_image(result_image, result_pixbuf, &data);
        },
        GuiMessage::RenderLines(lines) => {
            horizontal_line(&select_pixbuf.borrow(), lines.y1);
            horizontal_line(&select_pixbuf.borrow(), lines.y2);
            vertical_line(&select_pixbuf.borrow(), lines.x1);
            vertical_line(&select_pixbuf.borrow(), lines.x2);
            let inner: &Pixbuf = &select_pixbuf.borrow();
            select_image.set_from_pixbuf(Some(inner));
        }
    }
}

fn update_image(image: Image, pixbuf: Rc<RefCell<Pixbuf>>, data: &ImgBuf<Rgba>) {
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