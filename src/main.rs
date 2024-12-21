use std::cell::Cell;
use std::rc::Rc;

use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider};
use gtk::{glib, ApplicationWindow, Button, Label, Box};

const APP_ID: &str = "org.bisheralmasri.helloworld";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();


    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    let num = Cell::new(0);

    let label = Rc::new(Label::builder()
        .label(&num.get().to_string())
        .build());

    let button = Button::builder()
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .margin_top(12)
        .label("Click here")
        .build();

    let label_clone = Rc::clone(&label);
    button.connect_clicked(move |_| {
        num.set(num.get() + 1);
        label_clone.set_text(&num.get().to_string());
    });

    let container = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .build();

    label.set_halign(gtk::Align::Center);
    label.set_valign(gtk::Align::Center);

    container.append(&*label);
    container.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Title")
        .child(&container)
        .default_width(200)
        .default_height(150)
        .build();

    window.set_size_request(175, 50);
    window.present();
}
