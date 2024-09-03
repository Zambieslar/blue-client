use gtk4::{
    prelude::*, Align, Application, ApplicationWindow, Box, CenterBox, Frame, Grid, Label,
    ScrolledWindow, Switch, TextBuffer, TextView, Window,
};

pub struct MainWindow {
    pub mgrid: Grid,
    pub dev_grid: Grid,
    pub window: ApplicationWindow,
    pub title: Label,
    pub title_frame: Frame,
    pub dev_list: Box,
    pub scrollable_dev: ScrolledWindow,
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

pub struct DevWindow {
    pub window: Window,
    pub mgrid: Grid,
    pub label: Label,
    pub connected: Switch,
    pub c_box: CenterBox,
    pub prop_box: TextView,
    pub prop_buffer: TextBuffer,
}

impl DevWindow {
    pub fn open() -> Self {
        let switch = Switch::builder().build();
        let dev_window = Self {
            window: Window::builder()
                .width_request(500)
                .height_request(700)
                .resizable(false)
                .build(),
            mgrid: Grid::builder().build(),
            connected: Switch::builder().build(),
            c_box: CenterBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .vexpand(true)
                .halign(Align::Fill)
                .build(),
            label: Label::builder()
                .label("Device")
                .justify(gtk4::Justification::Center)
                .build(),
            prop_box: TextView::builder()
                .width_request(500)
                .height_request(300)
                .build(),
            prop_buffer: TextBuffer::new(None),
        };
        dev_window
            .prop_box
            .set_buffer(Some(&dev_window.prop_buffer));
        dev_window.mgrid.attach(&dev_window.c_box, 0, 0, 500, 700);
        dev_window.c_box.set_start_widget(Some(&dev_window.label));
        dev_window
            .c_box
            .set_center_widget(Some(&dev_window.connected));
        dev_window.c_box.set_end_widget(Some(&dev_window.prop_box));
        dev_window.window.set_child(Some(&dev_window.mgrid));

        dev_window
    }
}
