use relm4::{tokio::task, Component, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use adw::prelude::*;
use std::{str, io::{BufReader, BufRead}, os::unix::thread, process::{Command,Stdio}, time::Duration};

pub struct ProcessModel {
    visible: bool,
    info: gtk::TextBuffer,
    output: gtk::TextBuffer,
    errors: gtk::TextBuffer
}

#[derive(Debug)]
pub enum ProcessMsg {
    Run(String),
    Show,
    Hide,
}

#[derive(Debug)]
pub enum CommandMsg {
    StreamInfo(String),
    StreamOutput(String),
    StreamErrors(String)
}

#[relm4::component(pub)]
impl Component for ProcessModel {
    type Init = ();
    type Input = ProcessMsg;
    type Output = ();
    type CommandOutput = CommandMsg;

    view! {
        gtk::Dialog {
            set_title: Some("Applet"),
            set_default_width: 600,
            set_default_height: 300,
            #[watch]
            set_visible: model.visible,
            
            gtk::Notebook {
                append_page[Some::<&gtk::Label>(&gtk::Label::new(Some("Info")))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,
        
                    gtk::TextView {
                        set_vexpand: true,
    
                        #[watch]
                        set_buffer: Some(&model.info),
                        set_editable: false,
                    }
                },
                append_page[Some::<&gtk::Label>(&gtk::Label::new(Some("Output")))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,
    
                    gtk::ScrolledWindow {
                        set_vexpand: true,
        
                        gtk::TextView {
                            set_vexpand: true,
        
                            #[watch]
                            set_buffer: Some(&model.output),
                            set_monospace: true,
                            set_editable: false,
                        }
                    }
                },
                append_page[Some::<&gtk::Label>(&gtk::Label::new(Some("Errors")))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,
    
                    gtk::ScrolledWindow {
                        set_vexpand: true,
        
                        gtk::TextView {
                            set_vexpand: true,
        
                            #[watch]
                            set_buffer: Some(&model.errors),
                            set_monospace: true,
                            set_editable: false,
                        }
                    }
                },

                set_current_page: Some(1)
            }


        }
    }

    // Initialize the component.
     fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProcessModel { visible: false, info: gtk::TextBuffer::default(), output: gtk::TextBuffer::default(), errors: gtk::TextBuffer::default() };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

     fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, 
        _: &Self::Root,) {
        match msg {
            ProcessMsg::Run(input) => {
                self.output.set_text("");
                self.errors.set_text("");
                
                let parts = input.split_ascii_whitespace().collect::<Vec<&str>>();
                let name = parts.first().unwrap();
                let args = &parts[1..];

                let mut command = Command::new(name);
                let command = command.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

                let mut process = command.spawn();
                match process {
                    Err(error) => {
                        self.info.set_text(format!("Error: {}",  &error).as_str());
                    }
                    Ok(mut process) => {
                        self.info.set_text(format!("Running: {}", input).as_str());

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
        
                        sender.spawn_command(move |out| {
                            if let Some(errors) = process.stderr.as_mut() {
                                let mut lines = BufReader::new(errors).lines();
                                for line in lines {
                                    let line = line.unwrap() + "\n";
                                    out.send(CommandMsg::StreamErrors(line)).unwrap();
                                };
                            };
                        });
                    }
                }


            }
            ProcessMsg::Show => {
                self.visible = true;
            }
            ProcessMsg::Hide => {
                self.visible = false;
            }
        }
    }

     fn update_cmd(
            &mut self,
            msg: Self::CommandOutput,
            sender: ComponentSender<Self>,
            _: &Self::Root,
        ) {
        match msg {
            CommandMsg::StreamInfo(text) => {
                self.info.insert(&mut self.info.iter_at_offset(-1), text.as_str());
            }
            CommandMsg::StreamOutput(text) => {
                self.output.insert(&mut self.output.iter_at_offset(-1), text.as_str());
            }
            CommandMsg::StreamErrors(text) => {
                self.errors.insert(&mut self.errors.iter_at_offset(-1), text.as_str());
            }
        }
    }
}
