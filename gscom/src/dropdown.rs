use gtk::{prelude::*, *};

//#[derive(Default, glib::Boxed, Clone)]
//#[boxed_type(name = "DropDownItem")]
#[derive(Default, Clone)]
pub struct DropDownItem {
    pub label: String,
    pub value: u16,
}

impl DropDownItem {
    pub fn new(label: &str, value: u16) -> Self {
        Self {
            label: label.to_string(),
            value,
        }
    }
}

pub struct DropDownRow {
    pub main_box: Box,
    port_label: Label,
    port_select: DropDown,
}

impl DropDownRow {
    pub fn build_row<T>(label: &str, items: &Vec<(String, T)>) -> Self 
    where T : Copy {
        let label = Label::new(Some(label));

        let mut list_items: Vec<&str> = Vec::new();
        // Wrap tuples in Object (you can use glib::BoxedAnyObject)
        for item in items {
            list_items.push(&item.0);
        }


        let model = StringList::new(&list_items);
        let dropdown = DropDown::builder().build();
        dropdown.set_model(Some(&model));

        dropdown.set_selected(0);

        // Handle selection changes
        dropdown.connect_selected_notify(move |dropdown| {
            let selected = dropdown.selected();
            println!("Selected: {}", selected); // String and u16 values
        });

        let hbox = Box::new(Orientation::Horizontal, 0);
        hbox.append(&label);
        hbox.append(&dropdown);

        DropDownRow {
            main_box: hbox,
            port_label: label,
            port_select: dropdown,
        }
    }

    /*
    pub fn build_row2(items: &Vec::<DropDownItem>) -> Self {        // port/device 
        let label = Label::new(Some("Device"));

        // Create a ListStore to hold the tuples
        let store = ListStore::new();

        for item in items {
            //store.append(&BoxedAnyObject::new(item));
            let wrapped_item = BoxedAnyObject::new(item);
            store.append(wrapped_item.upcast_ref::<Object>());
        }

        let dropdown = DropDown::builder().build();
        dropdown.set_model(Some(&store));

        /*
        let sl = StringList::new(values);
        let exp = PropertyExpression::new(
            gtk::StringObject::static_type(),
            None::<gtk::Expression>,
            "string",
        );
        */

        // Create a factory for rendering the items in the DropDown
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_factory, item: &Object| {
            let list_item = item.downcast_ref::<ListItem>().unwrap();
            let label = Label::new(None);
            list_item.set_child(Some(&label));
        });

        factory.connect_bind(move |_factory, list_item: &Object| {
            let item = list_item.downcast_ref::<ListItem>().unwrap();
            let item_data = item.item().and_then(|obj| obj.downcast_ref::<BoxedAnyObject>()).unwrap();// . obj.downcast::<DropDownItem>().ok()).unwrap();
            let label = item.child().unwrap().downcast::<Label>().unwrap();
            let a = item_data.downcast_ref::<DropDownItem>();
            label.set_text(&item_data.label); // Set label from the tuple's string
        });

        dropdown.set_factory(Some(&factory));

        // Handle selection changes
        dropdown.connect_selected_notify(move |dropdown| {
            if let Some(selected) = dropdown.selected_item() {
                let item: DropDownItem = selected.downcast_ref::<DropDownItem>().unwrap().clone();
                println!("Selected: {} with value: {}", item.label, item.value);
            }
        });


        let hbox = Box::new(Orientation::Horizontal, 0);

        hbox.append(&label);
        hbox.append(&dropdown);

        // baud rate

        ComboRow {
            main_box: hbox,
            port_label: label,
            port_select: dropdown,
        }

    } */
}
