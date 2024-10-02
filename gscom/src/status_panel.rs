use std::{borrow::Borrow, cell::Cell, rc::Rc};

use gtk::{prelude::*, *};
use crate::panel::PanelTrait;

pub struct StatusPanel<'a> {
    sent_count: Rc<Cell<u64>>,
    recv_count: Rc<Cell<u64>>,
    sent_text1: Option<&'a Text>,
    recv_text1: Option<&'a Text>,
    button: Option<Button>
}


impl<'a> StatusPanel<'a> {
    fn increase_send_count(&mut self, count: u64) {
        //self.sent_count += count;
        self.sent_count.set(self.sent_count.get() + count);
    }

    fn increase_recv_count(&mut self, count: u64) {
        self.recv_count.set(self.recv_count.get() + count);
    }
}

impl<'a> PanelTrait for StatusPanel<'a> {
    fn new() -> Self {
        StatusPanel {
            sent_count: Rc::new(Cell::new(0)),
            recv_count: Rc::new(Cell::new(0)),
            sent_text1: None,
            recv_text1: None,
            button: None
        }
    }

    fn build_panel(&self) -> Widget {
        let image = Image::from_file("assets/unplug.svg");
        let sent_label = Label::new(Some("Send"));
        let sent_text = Text::builder().text(self.sent_count.get().to_string()).build();
        //self.sent_text1 = Text::builder().text(format!("{}", self.sent_count.get())).build();
        //self.sent_text1.set_text("0");
        let recv_label = Label::new(Some("Recv"));
        let recv_text = Text::builder().text(self.recv_count.get().to_string()).build();
        //self.recv_text1 = Text::builder().text(format!("{}", self.recv_count.get())).build();
        //self.recv_text1.set_text("0");

        let button = Button::builder()
            .label("clear")
            .margin_start(12)
            .margin_end(12)
            .build();

        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.append(&image);
        hbox.append(&sent_label);
        hbox.append(&sent_text);
        hbox.append(&recv_label);
        hbox.append(&recv_text);
        hbox.append(&button);

        //let st = self.sent_text.clone();
        //let rt = self.recv_text.clone();
        let sc = self.sent_count.clone();
        let rc = self.recv_count.clone();
        //self.sent_text1 = Some(&sent_text);
        //self.recv_text1 = Some(&recv_text);
        //let aa: &Text = self.sent_text1.borrow();
        //let bb: &Text = self.recv_text1.borrow();
        button.connect_clicked(move |_| {
            /*
            let data = b"Test data";
            let mut conn = connection.borrow_mut();
            match conn.write_data(data) {
                Ok(_) => buffer.set_text("Data sent successfully"),
                Err(e) => buffer.set_text(&format!("Failed to send data: {:?}", e)),
            }
            */
            //sent_text.set_text("0");
           //recv_text.set_text("0");
            sc.set(0);
            rc.set(0);
            sent_text.set_text("0");
            recv_text.set_text("0");
            //aa.set_text("aa");
            //bb.set_text("aa");
        });

        hbox.into()
    }
}
