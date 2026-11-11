use adw::prelude::*;
use gtk::prelude::*;

fn main() {
    let app = adw::Application::builder()
        .application_id("lu.parkus.rufly")
        .build();

    app.connect_activate(|app| {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Rufly")
            .default_width(750)
            .default_height(500)
            .build();

        window.present();
    });

    app.run();
}