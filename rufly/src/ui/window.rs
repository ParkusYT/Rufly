use adw::{HeaderBar, prelude::*};
use gtk::prelude::*;

pub fn build(app: &adw::Application) {
    // Main app window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Rufly")
        .default_width(760)
        .default_height(600)
        .build();

    // Header bar
    let header = adw::HeaderBar::new();

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.append(&header);

    let title = gtk::Label::new(Some("Rufly"));
    title.add_css_class("heading");
    header.set_title_widget(Some(&title));

    let menu_button = gtk::Button::from_icon_name("open-menu-symbolic");
    menu_button.set_tooltip_text(Some("Application menu"));
    header.pack_end(&menu_button);

    // Main content
    let content = gtk::Box::new(gtk::Orientation::Vertical, 20);

    content.set_margin_top(32);
    content.set_margin_bottom(32);
    content.set_margin_start(32);
    content.set_margin_end(32);

    root.append(&content);
    window.set_content(Some(&root));
    window.present();

    // Page heading
    let heading = gtk::Label::new(Some("Create bootable media"));
    heading.add_css_class("title-1");
    heading.set_halign(gtk::Align::Start);

    let subtitle = gtk::Label::new(Some("Create bootable USB drives from ISO images"));
    subtitle.add_css_class("dim-label");
    subtitle.set_halign(gtk::Align::Start);

    content.append(&heading);
    content.append(&subtitle);

    // ISO selection
    let iso_card = gtk::Frame::new(None);
    iso_card.add_css_class("card");

    let iso_content = gtk::Box::new(gtk::Orientation::Vertical, 12);

    iso_content.set_margin_top(32);
    iso_content.set_margin_bottom(32);
    iso_content.set_margin_start(32);
    iso_content.set_margin_end(32);
    iso_content.set_halign(gtk::Align::Center);

    let iso_icon = gtk::Image::from_icon_name("document-open-symbolic");
    iso_icon.set_pixel_size(48);

    let iso_title = gtk::Label::new(Some("Select an ISO image"));
    iso_title.add_css_class("title-2");

    let iso_description = gtk::Label::new(Some("Choose an operating system image to get started"));
    iso_description.add_css_class("dim-label");
    iso_description.set_wrap(true);
    iso_description.set_justify(gtk::Justification::Center);

    let iso_button = gtk::Button::with_label("ISO selection");
    iso_button.set_sensitive(false);

    iso_content.append(&iso_icon);
    iso_content.append(&iso_title);
    iso_content.append(&iso_description);
    iso_content.append(&iso_button);

    iso_card.set_child(Some(&iso_content));

    content.append(&iso_card);

    // USB device selection
    let device_heading = gtk::Label::new(Some("Target device"));
    device_heading.add_css_class("heading");
    device_heading.set_halign(gtk::Align::Start);

    let device_card = gtk::Frame::new(None);
    device_card.add_css_class("card");

    let device_content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    device_content.set_margin_top(16);
    device_content.set_margin_bottom(16);
    device_content.set_margin_start(16);
    device_content.set_margin_end(16);

    let usb_icon = gtk::Image::from_icon_name("drive-removable-media-symbolic");
    usb_icon.set_pixel_size(32);

    let device_info = gtk::Box::new(gtk::Orientation::Vertical, 4);

    let device_title = gtk::Label::new(Some("No USB drive detected"));
    device_title.set_halign(gtk::Align::Start);

    let device_description = gtk::Label::new(Some("Connect a USB drive to continue."));
    device_description.add_css_class("dim-label");
    device_description.set_halign(gtk::Align::Start);

    device_info.append(&device_title);
    device_info.append(&device_description);

    device_content.append(&usb_icon);
    device_content.append(&device_info);

    device_card.set_child(Some(&device_content));

    content.append(&device_heading);
    content.append(&device_card);

    // Flash button
    let flash_button = gtk::Button::with_label("Start flashing");
    flash_button.add_css_class("suggested-action");
    flash_button.add_css_class("pill");
    flash_button.set_sensitive(false);

    content.append(&flash_button);

    // Status
    let status = gtk::Label::new(Some("Ready · No operation in progress"));
    status.add_css_class("dim-label");
    status.set_halign(gtk::Align::Center);

    content.append(&status);
}
