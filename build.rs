fn main() {
    relm4_icons_build::bundle_icons(
        "icon_names.rs",
        Some("dev.bricked.applets"),
        None::<&str>,
        None::<&str>,
        ["play", "tab-new", "menu-large"],
    );
}