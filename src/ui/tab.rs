use relm4::{prelude::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque}, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent};
use adw::prelude::*;
use gtk::{prelude::*, EntryBuffer, TextBuffer};
use std::{str, io::{BufReader, BufRead}, os::unix::thread, process::{Command,Stdio}, time::Duration};

#[derive(Debug)]
pub enum TabModel {
    Entry{
        command: gtk::EntryBuffer
    },
    Result{
        command: String,
        output: gtk::TextBuffer,
    }
}

#[derive(Debug)]
pub enum TabInit {
    Entry{},
    Result{ command: String, }
}

#[derive(Debug)]
pub enum TabMsg {
    CreateResult
}

#[derive(Debug)]
pub enum TabOutput {
    CreateTab(TabInit)
}

#[derive(Debug)]
pub enum CommandMsg {
    Init,
    StreamOutput(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for TabModel {
    type Init = TabInit;
    type Input = TabMsg;
    type Output = TabOutput;
    type CommandOutput = CommandMsg;
    type ParentWidget = adw::TabView;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            
            match self {
                TabModel::Entry{ command } => {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,

                        gtk::Entry {
                            set_placeholder_text: Some("Enter a command"),
                            #[watch]
                            set_buffer: command,
                            connect_activate => TabMsg::CreateResult
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 5,
                            set_visible: false,
                        },

                        gtk::Button {
                            set_label: "Run",
                            set_icon_name: "play",
                            connect_clicked => TabMsg::CreateResult
                        },
                    }
                },
                TabModel::Result{ command, output } => {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,

                        gtk::TextView {
                            set_vexpand: true,
        
                            #[watch]
                            set_buffer: Some(output),
                            set_monospace: true,
                            set_editable: false,
                        },
                    }
                },
            },
        },
        
        #[local_ref]
        returned_widget -> adw::TabPage {
            #[watch]
            set_title: match self {
                TabModel::Entry{ .. } => "New Tab",
                TabModel::Result { command, ..} => command.as_str(),
            },
        },
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let model = match value {
            TabInit::Result{command} => Self::Result {
                command,
                output: TextBuffer::default(),
            },
            TabInit::Entry{} => Self::Entry {
                command: EntryBuffer::default()
            }
        };

        sender.spawn_oneshot_command(|| CommandMsg::Init);
        model
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            TabMsg::CreateResult => {
                sender.output(TabOutput::CreateTab(TabInit::Result{
                    command: match self {
                        TabModel::Entry {command} => command.text().to_string(),
                        TabModel::Result {command, .. } => command.clone(),
                    }
                })).unwrap();
            }
        }
    }

    fn update_cmd(&mut self, message: Self::CommandOutput, sender: FactorySender<Self>) {
        if let TabModel::Result{ command, output } = self {
            match message {
                CommandMsg::StreamOutput(text) => {
                    output.insert(&mut output.iter_at_offset(-1), text.as_str());
                },
                CommandMsg::Init => {
                    let parts = command.split_ascii_whitespace().collect::<Vec<&str>>();
                    let (&[name], args) = parts.split_at(1) else {
                        unreachable!();
                    };

                    let mut command = Command::new(name);
                    let command = command.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

                    let mut process = command.spawn();
                    let Ok(mut process) = process else {
                        panic!("command failed!");
                    };

                    sender.spawn_command(move |out| {
                        if let Some(output) = process.stdout.as_mut() {
                            let mut lines = BufReader::new(output).lines();
                            for line in lines {
                                let line = line.unwrap() + "\n";
                                // TODO: Implement ANSI escape codes
                                let line = String::from_utf8(strip_ansi_escapes::strip(line.as_bytes())).unwrap();
                                out.send(CommandMsg::StreamOutput(line)).unwrap();
                            };
                        };
                    });
                }
            }
        }
    }
}