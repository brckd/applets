use relm4::{Component, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent, ComponentController};
use adw::prelude::*;
use std::{str, io::{BufReader, BufRead}, os::unix::thread, process::{Command,Stdio}, time::Duration};
use super::process::{ProcessModel, ProcessMsg};

pub struct AppModel {
    command: gtk::EntryBuffer,
    process: Controller<ProcessModel>
}

#[derive(Debug)]
pub enum AppMsg {
    Run,
    Nothing,
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
        adw::ApplicationWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                adw::HeaderBar {},

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Entry {
                        set_placeholder_text: Some("Enter a command"),
                        set_buffer: &model.command,
                        set_hexpand: true,
                        connect_activate => AppMsg::Run,
                    },

                    gtk::Button {
                        set_label: "Run",
                        connect_clicked => AppMsg::Run,
                    },
                }
            }
        }
    }

    // Initialize the component.
     fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let process = ProcessModel::builder()
            .transient_for(&root)
            .launch(()).forward(sender.input_sender(), |()| AppMsg::Nothing);

        let model = AppModel { command: gtk::EntryBuffer::default(), process };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

     fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, 
        _: &Self::Root,) {
        match msg {
            AppMsg::Run => {
                let command = self.command.text().to_string();
                self.process.emit(ProcessMsg::Show);
                self.process.emit(ProcessMsg::Run(command));
            }
            AppMsg::Nothing => {}
        }
    }
}
