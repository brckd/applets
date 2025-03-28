use adw::Application;
use relm4::RelmApp;
use applets::ui::app::AppModel;

fn main() {
    let app = Application::builder()
        .application_id("dev.bricked.applets")
        .build();
    let app = RelmApp::from_app(app);
    app.run::<AppModel>(());
}
