use adw::prelude::*;

pub fn run() {
    let app = adw::Application::builder()
        .application_id("lu.parkus.Rufly")
        .build();

    app.connect_activate(|app| {
        crate::ui::window::build(app);
    });

    app.run();
}
