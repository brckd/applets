use applets::ui::app::AppModel;

mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() {
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    let app = adw::Application::builder()
        .application_id("dev.bricked.applets")
        .build();
    let app = relm4::RelmApp::from_app(app);
    app.run::<AppModel>(());
}
