use gtk::prelude::*;
use gtk::{Box, Orientation, Image, Button, Frame, Widget};

use scom::SerialConnection;
use scom::BaudRate;

use crate::panel::PanelTrait;
use crate::dropdown::DropDownRow;

pub struct PortPanel;

impl PanelTrait for PortPanel {
    fn new() -> Self {
        PortPanel {}
    }

    fn build_panel(&self) -> Widget {
        // Device/port
        // list available serial devices/ports
        let ports = SerialConnection::list_ports();

        let ports = match ports {
            Ok(ps) => {
                println!("{:?}", ps);
                let mut a = Vec::new();
                for p in ps {
                    if let Some(path) = p.to_str() {
                        //a.push(DropDownItem{label: path.to_string(), value: 0u16});   // TODO
                        a.push((path.to_string(), 0u16));   // TODO
                    }
                }
                a
            },
            Err(err) => {
                println!("Cannot list serial port at present: {}", err);
                Vec::new()
            }
        };

        let frame = Frame::new(Some(""));
        let vbox = Box::new(Orientation::Vertical, 0);
        let row = DropDownRow::build_row("Device", &ports);

        vbox.append(&row.main_box);

        // baud rate
        let baud_rates = BaudRate::values();

        let row = DropDownRow::build_row("Baud Rate", &baud_rates);
        vbox.append(&row.main_box);

        let hbox = Box::new(Orientation::Horizontal, 0);
        let image = Image::from_file("assets/unplug.svg");
        let port_button = Button::builder()
            .label("Open Port")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        hbox.append(&image);
        hbox.append(&port_button);
        vbox.append(&hbox);

        frame.set_child(Some(&vbox));
        frame.into()
    }
}
