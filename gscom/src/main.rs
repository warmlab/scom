mod menubar;
mod panel;
mod input_panel;
mod output_panel;
mod port_panel;
mod dropdown;
mod status_panel;

use std::rc::Rc;
use std::cell::RefCell;

use gdk::Display;
use gtk::Application;
use gtk::{*, prelude::*};

use glib::ExitCode;

use input_panel::InputPanel;
use scom::SerialConnection;
use status_panel::StatusPanel;

use crate::output_panel::OutputPanel;
use crate::panel::PanelTrait;

const APP_ID: &str = "work.arcticcircle.tool.searial";

//fn build_ui(app: &Application, connection: Rc<RefCell<SerialConnection>>) {
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
                                   .application(app)
                                   .title("Serial Communication Tool")
                                   .build();
    window.set_default_size(800, 600);

    let provider = CssProvider::new();
    //provider.load_from_file("style.css");
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let button = Button::builder()
        .label("Send Data")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |_| {
        /*
        let data = b"Test data";
        let mut conn = connection.borrow_mut();
        match conn.write_data(data) {
            Ok(_) => buffer.set_text("Data sent successfully"),
            Err(e) => buffer.set_text(&format!("Failed to send data: {:?}", e)),
        }
        */
    });

    // create menu bar
    let menubar = crate::menubar::create_menubar();
    app.set_menubar(Some(&menubar));

    let vbox = Box::new(Orientation::Vertical, 5);
    //let output_panel = ;
    vbox.append(&OutputPanel::build_panel());
    vbox.append(&InputPanel::build_panel());
    vbox.append(&StatusPanel::build_panel());
    //vbox.append(&hbox);
    vbox.append(&button);
    //window.set_child(Some(&hbox));
    window.set_child(Some(&vbox));
    //window.shows_menubar();
    window.set_show_menubar(true);
    window.present();
}


fn connect() {
    let connection = Rc::new(RefCell::new(SerialConnection::new("/dev/ttyS1", 9600).unwrap()));
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
