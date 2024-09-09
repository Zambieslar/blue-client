use bluer::Address;
use gtk4::{
    prelude::*, Align, Application, ApplicationWindow, Box, CenterBox, Frame, Grid, Label,
    ScrolledWindow, Switch, TextBuffer, TextView, Window,
};

pub struct DevStatus {
    pub remote_address: Label,
    pub paired: Label,
    pub connected: Label,
    pub trusted: Label,
    pub blocked: Label,
    pub tx_power: Label,
    pub battery_percentage: Label,
}

pub struct MainWindow {
    pub mgrid: Grid,
    pub dev_grid: Grid,
    pub window: ApplicationWindow,
    pub title: Label,
    pub title_frame: Frame,
    pub dev_list: Box,
    pub scrollable_dev: ScrolledWindow,
}

pub struct DevWindow {
    pub window: Window,
    pub mgrid: Grid,
    pub label: Label,
    pub status_label: Label,
    pub connected: Switch,
    pub c_box: CenterBox,
    pub box_toggle: Box,
    pub prop_box: TextView,
    pub prop_buffer: TextBuffer,
    pub dev_status: DevStatus,
}

impl MainWindow {
    pub fn new(app: Application) -> Self {
        let pmgrid = Grid::builder()
            .width_request(300)
            .height_request(700)
            .build();
        let pdev_grid = Grid::builder()
            .width_request(300)
            .height_request(650)
            .build();
        let pwindow = ApplicationWindow::builder()
            .application(&app)
            .resizable(false)
            .width_request(300)
            .height_request(700)
            .child(&pmgrid)
            .build();
        let ptitle = Label::builder()
            .width_request(300)
            .height_request(50)
            .justify(gtk4::Justification::Center)
            .label("Devices")
            .build();
        let ptitle_frame = Frame::builder()
            .height_request(50)
            .width_request(300)
            .focusable(false)
            .label_widget(&ptitle)
            .build();
        let pdev_list = Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();
        let pscrollable_dev = ScrolledWindow::builder()
            .width_request(300)
            .height_request(650)
            .child(&pdev_list)
            .build();
        Self {
            mgrid: pmgrid,
            dev_grid: pdev_grid,
            window: pwindow,
            title: ptitle,
            title_frame: ptitle_frame,
            dev_list: pdev_list,
            scrollable_dev: pscrollable_dev,
        }
    }
}

impl DevWindow {
    pub fn open() -> Self {
        let dev_window = Self {
            window: Window::builder()
                .width_request(300)
                .height_request(500)
                .resizable(false)
                .build(),
            mgrid: Grid::builder().build(),
            connected: Switch::builder().build(),
            c_box: CenterBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .vexpand(true)
                .hexpand(true)
                .halign(Align::Fill)
                .build(),
            box_toggle: Box::builder()
                .orientation(gtk4::Orientation::Vertical)
                .halign(Align::Center)
                .valign(Align::Start)
                .build(),
            label: Label::builder()
                .label("Device")
                .justify(gtk4::Justification::Center)
                .build(),
            status_label: Label::builder()
                .label("Connected")
                .justify(gtk4::Justification::Center)
                .build(),
            prop_box: TextView::builder()
                .width_request(500)
                .height_request(300)
                .build(),
            prop_buffer: TextBuffer::new(None),
            dev_status: DevStatus {
                remote_address: Label::new(Some("Address:")),
                paired: Label::new(Some("Paired:")),
                connected: Label::new(Some("Connected:")),
                trusted: Label::new(Some("Trusted:")),
                blocked: Label::new(Some("Blocked:")),
                tx_power: Label::new(Some("Power:")),
                battery_percentage: Label::new(Some("Battery:")),
            },
        };
        dev_window
            .prop_box
            .set_buffer(Some(&dev_window.prop_buffer));
        dev_window.mgrid.attach(&dev_window.c_box, 0, 0, 300, 500);
        dev_window.c_box.set_start_widget(Some(&dev_window.label));
        dev_window
            .c_box
            .set_center_widget(Some(&dev_window.box_toggle));
        dev_window.box_toggle.append(&dev_window.connected);
        dev_window.box_toggle.append(&dev_window.status_label);
        dev_window.window.set_child(Some(&dev_window.mgrid));

        dev_window
    }
}
