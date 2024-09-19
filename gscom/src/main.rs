use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, TextView};
use std::rc::Rc;
use std::cell::RefCell;
use scom::SerialConnection;

fn build_ui(app: &Application, connection: Rc<RefCell<SerialConnection>>) {
    let window = ApplicationWindow::new(app);
    window.set_title("Serial COM1 Tool");
    window.set_default_size(400, 300);

    let text_view = TextView::new();
    let buffer = text_view.buffer().expect("Failed to get buffer");

    let button = Button::with_label("Send Data");
    button.connect_clicked(move |_| {
        let data = b"Test data";
        let mut conn = connection.borrow_mut();
        match conn.write_data(data) {
            Ok(_) => buffer.set_text("Data sent successfully"),
            Err(e) => buffer.set_text(&format!("Failed to send data: {:?}", e)),
        }
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&text_view, true, true, 0);
    vbox.pack_start(&button, false, false, 0);

    window.add(&vbox);
    window.show_all();
}

fn main() {
    let application = Application::new(Some("com.example.serial"), Default::default());
    
    let connection = Rc::new(RefCell::new(SerialConnection::new("/dev/ttyS1", 9600).unwrap()));

    application.connect_activate(move |app| {
        build_ui(app, Rc::clone(&connection));
    });

    application.run();
}
