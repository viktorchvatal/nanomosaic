use gtk::*;

pub fn create_image() -> Image {
    let image = Image::new();
    image.set_halign(gtk::Align::Start);
    image.set_valign(gtk::Align::Start);
    image
}

pub fn create_window(application: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(application);
    window.set_title("Mosaic creator");
    window.set_border_width(10);
    window.set_position(WindowPosition::Center);
    window.set_default_size(1600, 900);
    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });
    window
}

pub fn create_dummy_scroller() -> ScrolledWindow {
    let scroller = gtk::ScrolledWindow::new(
        gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT
    );
    scroller.set_policy(PolicyType::External, PolicyType::External);
    scroller
}