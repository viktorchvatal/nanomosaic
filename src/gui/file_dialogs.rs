use gtk::*;

pub fn open_file_dialog(window: ApplicationWindow) -> Option<String> {
    let open_dialog = FileChooserDialog::with_buttons(
        "Load image", Some(&window), FileChooserAction::Open,
        &[
            ("_Cancel", ResponseType::Cancel), 
            ("_Open", ResponseType::Accept)
        ]
    );

    open_dialog.add_filter(&create_image_filter());

    let result = open_dialog.clone().run();    
    open_dialog.close();

    match result {
        -3 => Some(open_dialog.get_filename()?.to_str()?.to_owned()),
        _ => None
    }
}

pub fn save_file_dialog(window: ApplicationWindow) -> Option<String> {
    let save_dialog = FileChooserDialog::with_buttons(
        "Save PNG image", Some(&window), FileChooserAction::Save,
        &[
            ("_Cancel", ResponseType::Cancel), 
            ("_Save", ResponseType::Accept)
        ]
    );

    save_dialog.add_filter(&create_image_filter());
    save_dialog.set_do_overwrite_confirmation(true);

    let result = save_dialog.clone().run();    
    save_dialog.close();

    match result {
        -3 => Some(save_dialog.get_filename()?.to_str()?.to_owned()),
        _ => None
    }
}

fn create_image_filter() -> FileFilter {
    let filter = FileFilter::new();
    filter.add_mime_type("image/png");
    filter.add_mime_type("image/jpeg");
    filter.add_mime_type("image/jpg");    
    filter.set_name("Image files (PNG, JPG)");
    filter
}
