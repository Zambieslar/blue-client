use std::time::Duration;

use gtk4::{
    prelude::{ApplicationExt, *},
    Application, Box, Button, Window,
};
use gtk4_layer_shell::{Edge, LayerShell};
use std::sync::mpsc::*;

use crate::{bluetooth::DeviceProperties, ui::*};

pub fn init_ui<'a>(
    app: Application,
    rx: std::sync::Arc<Receiver<DeviceProperties>>,
) -> Application {
    app.connect_activate(move |app| {
        let main_window = MainWindow::new(app.clone());
        main_window
            .dev_grid
            .attach(&main_window.scrollable_dev, 0, 0, 1, 1);
        main_window
            .mgrid
            .attach(&main_window.dev_grid, 0, 50, 300, 650);
        main_window
            .mgrid
            .attach(&main_window.title_frame, 0, 0, 300, 50);
        main_window.window.init_layer_shell();
        main_window.window.set_anchor(Edge::Right, true);
        main_window.window.set_anchor(Edge::Top, true);
        main_window.window.present();
        let rx = std::sync::Arc::clone(&rx);

        gtk4::glib::timeout_add_local(Duration::from_secs(3), move || match rx.try_recv() {
            Ok(device) => {
                let dev_button = Button::builder()
                    .label(device.name.unwrap_or("Unknown".to_string()))
                    .width_request(290)
                    .height_request(50)
                    .build();
                let dev_box = Box::builder()
                    .orientation(gtk4::Orientation::Vertical)
                    .height_request(50)
                    .valign(gtk4::Align::Start)
                    .width_request(290)
                    .build();
                dev_box.append(&dev_button);
                main_window.dev_list.append(&dev_box);
                dev_button.connect_clicked(move |_| {
                    let dev_window = DevWindow::open();
                    dev_window.window.present();
                });
                gtk4::glib::ControlFlow::Continue
            }
            _ => gtk4::glib::ControlFlow::Break,
        });
    });

    app
}
