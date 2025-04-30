use relm4::{prelude::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque}, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent};
use adw::prelude::*;
use gtk::prelude::*;
use std::{str, io::{BufReader, BufRead}, os::unix::thread, process::{Command,Stdio}, time::Duration};

#[derive(Debug)]
pub struct TabModel {
    command: gtk::EntryBuffer,
}

#[derive(Debug)]
pub struct TabMsg {
}

#[derive(Debug)]
pub enum TabOutput {
}

#[relm4::factory(pub)]
impl FactoryComponent for TabModel {
    type Init = ();
    type Input = TabMsg;
    type Output = TabOutput;
    type CommandOutput = ();
    type ParentWidget = adw::TabView;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,

            gtk::Entry {
                set_placeholder_text: Some("Enter a command"),
                set_buffer: &self.command,
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_visible: false,
            },

            gtk::Button {
                set_label: "Run",
                set_icon_name: "play",
            },
        },
        
        #[local_ref]
        returned_widget -> adw::TabPage {
            set_title: "New Tab",
        },
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { command: gtk::EntryBuffer::default(), }
    }
}