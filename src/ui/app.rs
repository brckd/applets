use std::{str, io::{BufReader, BufRead}, os::unix::thread, process::{Command,Stdio}, time::Duration};
use relm4::{prelude::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque}, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent};
use relm4::actions::{AccelsPlus, ActionablePlus, RelmAction, RelmActionGroup};
use adw::prelude::*;
use gtk::prelude::*;
use super::tab::*;

pub struct AppModel {
    overview_open: bool,
    tabs: FactoryVecDeque<TabModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenOverview,
    Nothing,
    CreateTab,
}

#[derive(Debug)]
pub enum CommandMsg {}

#[relm4::component(pub)]
impl Component for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type CommandOutput = CommandMsg;

    view! {
        #[root]
        adw::ApplicationWindow {
            set_title: Some("Applets"),
            set_default_width: 600,
            set_default_height: 300,

            adw::TabOverview {
                #[watch]
                set_open: model.overview_open,
                set_view: Some(&tab_view),
                set_enable_search: true,

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar {
                        pack_start = &gtk::Button {
                            set_icon_name: "tab-new",
                            connect_clicked => AppMsg::CreateTab,
                        },

                        pack_end = &gtk::Button {
                            set_icon_name: "view-grid-symbolic",
                            connect_clicked => AppMsg::OpenOverview,
                        },
                    },

                    add_top_bar = &adw::TabBar {
                        set_view: Some(&tab_view),
                    },

                    #[local_ref]
                    tab_view -> adw::TabView {
                        set_margin_all: 5,
                        set_margin_top: 0,
                    },
                },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tabs = FactoryVecDeque::builder()
            .launch(adw::TabView::default())
            .forward(sender.input_sender(), |output| AppMsg::Nothing);
        let model = AppModel { overview_open: false, tabs };
        
        let tab_view = model.tabs.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, 
        _: &Self::Root,) {
        let mut tabs_guard = self.tabs.guard();
        match msg {
            AppMsg::OpenOverview => {
                self.overview_open = true;
            }
            AppMsg::CreateTab => {
                self.overview_open = false;
                tabs_guard.push_back(());
            }
            AppMsg::Nothing => {
                self.overview_open = false;
            }
        }
    }
}
