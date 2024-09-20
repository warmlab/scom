use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, TextView};
use gtk::Box;

use glib::ExitCode;

use scom::SerialConnection;

const APP_ID: &str = "org.gtk_rs.tool.searial";

//fn build_ui(app: &Application, connection: Rc<RefCell<SerialConnection>>) {
fn build_ui(app: &Application) {
    
    let connection = Rc::new(RefCell::new(SerialConnection::new("/dev/ttyS1", 9600).unwrap()));
    let window = ApplicationWindow::builder()
                                            .application(app)
                                            .title("Serial Communication Tool")
                                            .build();
    window.set_default_size(400, 300);

    let text_view = TextView::new();
    let buffer = text_view.buffer(); //.expect("Failed to get buffer");

    let button = Button::builder()
        .label("Send Data")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |_| {
        let data = b"Test data";
        let mut conn = connection.borrow_mut();
        match conn.write_data(data) {
            Ok(_) => buffer.set_text("Data sent successfully"),
            Err(e) => buffer.set_text(&format!("Failed to send data: {:?}", e)),
        }
    });

    let vbox = Box::new(gtk::Orientation::Vertical, 5);
    vbox.append(&text_view);
    vbox.append(&button);

    window.set_child(Some(&vbox));
    window.present();
}

fn main() -> ExitCode {
    //let application = Application::new(Some("com.example.serial"), Default::default());
    let app = Application::builder().application_id(APP_ID).build();

    //application.connect_activate(move |app| {
    //    build_ui(app, Rc::clone(&connection));
    //});

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
