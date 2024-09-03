use std::sync::{mpsc, Arc};

use bluetooth::DeviceProperties;
use events::init_ui;
use gtk4::{prelude::*, Application};
use tokio::{self, runtime};

mod bluetooth;
mod events;
mod ui;

fn main() {
    let (tx, rx) = mpsc::sync_channel::<DeviceProperties>(25);
    let tx = Arc::new(tx);
    let rx = Arc::new(rx);

    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to initialize Tokio runtime");

    runtime.spawn(bluetooth::blue_init(tx));

    let app = Application::builder()
        .application_id("org.zambiebam.blueclient")
        .build();

    let app = init_ui(app, rx);

    app.run();
}
