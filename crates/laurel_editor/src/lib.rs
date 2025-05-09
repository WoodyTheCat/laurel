use core::buffer::Buffer;
use core::window::VirtualWindow;
use std::path::PathBuf;
use std::vec;

use async_lsp::lsp_types::Url;
use core::document::Document;
use highlighter::HighlighterConfig;
use tokio::sync::mpsc::error::SendError;
use tracing::{debug, info, warn};

use iced::keyboard::Key;
use iced::widget::scrollable::Scrollbar;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Font, Length, Padding, Renderer, Subscription, Task};

use laurel_common::{
    text::{CursorMessage, Position},
    Element, LaurelTheme,
};
use laurel_lsp::{
    LspClientNotification, LspCommand, LspConnection, LspMessage, LspServerNotification,
    Synchronise,
};

use rfd::FileDialog;

use widgets::modal::file_selector::Modal;
use widgets::textbox::Textbox;
use widgets::textbox_container::TextboxContainer;
use widgets::view_port::{ViewPort, ViewPortMessage};
use widgets::{layout, line_number};

pub mod core;
pub mod highlighter;
pub mod styles;
pub mod widgets;

#[derive(Debug, Clone)]
pub enum KeyEvent {
    Special(Key, Modifiers),
    CharacterReceived(char),
}

impl<T> From<Result<(), SendError<T>>> for Message {
    fn from(_: Result<(), SendError<T>>) -> Self {
        Message::SendError
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyEvent(KeyEvent),
    CursorEvent(Position),
    Offset(f32, f32),
    View(f32, f32),
    SelectionMove(Position),
    Paste(String),
    Open(String),
    DismissModal,
    FileFilter(String),
    SetTextBoxFocus(bool),
    CloseClient,

    // Client messages
    ClientStart(()),
    OpenDocument(()),
    CloseDocument(()),
    DocChanged(()),
    HasInitialized(()),
    DidSave(()),
    Done(()),
    None,

    // LSP messages
    LspNotification(LspClientNotification),
    LspMessage(LspMessage),

    // Menu Messages
    SelectFile,
    SelectFolder,
    Save,

    SendError,
}

impl Message {
    // fn from_keyrelease(modifiers: Modifiers) -> Option<Self> {
    //     Some(Message::KeyEvent(KeyEvent::Special(Key::None, modifiers)))
    // }
    // fn from_keycode(value: KeyCode, modifier: Modifiers) -> Option<Self> {
    //     match value {
    //         KeyCode::Up => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyUp, modifier))),
    //         KeyCode::Down => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyDown, modifier))),
    //         KeyCode::Right => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyRight, modifier))),
    //         KeyCode::Left => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyLeft, modifier))),
    //         KeyCode::Backspace => Some(Self::KeyEvent(KeyEvent::Special(
    //             Key::KeyBackSpace,
    //             modifier,
    //         ))),
    //         KeyCode::Delete => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyDelete, modifier))),
    //         KeyCode::Enter => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyEnter, modifier))),
    //         KeyCode::Tab => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyTab, modifier))),
    //         KeyCode::Escape => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyEsc, modifier))),
    //         KeyCode::Home => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyHome, modifier))),
    //         KeyCode::End => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyEnd, modifier))),
    //         KeyCode::PageUp => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyPgUp, modifier))),
    //         KeyCode::PageDown => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyPgDown, modifier))),
    //         KeyCode::Space => Some(Self::KeyEvent(KeyEvent::Special(Key::Key(' '), modifier))),
    //
    //         // Alphabet
    //         KeyCode::A => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyA, modifier))),
    //         KeyCode::B => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyB, modifier))),
    //         KeyCode::C => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyC, modifier))),
    //         KeyCode::D => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyD, modifier))),
    //         KeyCode::E => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyE, modifier))),
    //         KeyCode::F => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyF, modifier))),
    //         KeyCode::G => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyG, modifier))),
    //         KeyCode::H => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyH, modifier))),
    //         KeyCode::I => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyI, modifier))),
    //         KeyCode::J => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyJ, modifier))),
    //         KeyCode::K => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyK, modifier))),
    //         KeyCode::L => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyL, modifier))),
    //         KeyCode::M => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyM, modifier))),
    //         KeyCode::N => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyN, modifier))),
    //         KeyCode::O => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyO, modifier))),
    //         KeyCode::P => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyP, modifier))),
    //         KeyCode::Q => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyQ, modifier))),
    //         KeyCode::R => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyR, modifier))),
    //         KeyCode::S => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyS, modifier))),
    //         KeyCode::T => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyT, modifier))),
    //         KeyCode::U => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyU, modifier))),
    //         KeyCode::V => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyV, modifier))),
    //         KeyCode::W => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyW, modifier))),
    //         KeyCode::X => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyX, modifier))),
    //         KeyCode::Y => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyY, modifier))),
    //         KeyCode::Z => Some(Self::KeyEvent(KeyEvent::Special(Key::KeyZ, modifier))),
    //
    //         _ => Some(Message::KeyEvent(KeyEvent::Special(Key::None, modifier))),
    //     }
    // }
}

impl CursorMessage for Message {
    fn from_cursor_position(pos: Position) -> Self {
        Self::CursorEvent(pos)
    }
    fn from_selection_move(pos: Position) -> Self {
        Self::SelectionMove(pos)
    }
}

impl ViewPortMessage for Message {
    fn view_change(height: f32, width: f32) -> Self {
        Self::View(height, width)
    }
    fn dismiss_modal() -> Self {
        Self::DismissModal
    }
    fn set_textbox_focus(is_focused: bool) -> Self {
        Self::SetTextBoxFocus(is_focused)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Modifiers {
    ctrl: bool,
    shift: bool,
    _alt: bool,
}

impl From<iced::keyboard::Modifiers> for Modifiers {
    fn from(value: iced::keyboard::Modifiers) -> Self {
        Self {
            ctrl: value.control(),
            shift: value.shift(),
            _alt: value.alt(),
        }
    }
}

pub struct Editor {
    // last_event: Option<Key>,
    text_box: Option<Textbox>,
    modifiers: Modifiers,
    modal: Option<Modal>,
    // workspace: Option<PathBuf>,
    lsp: Option<LspConnection>,
    file_filter: String,
    // client_id: usize,
    theme: LaurelTheme,
}

impl Editor {
    // type Executor = executor::Default;
    // type Message = Message;
    // type Theme = iced::Theme;
    // type Flags = ();

    pub fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Editor {
                // last_event: None,
                text_box: None,
                modifiers: Modifiers::default(),
                modal: None,
                lsp: None,
                // workspace: None,
                file_filter: String::default(),
                // client_id: 1,
                theme: LaurelTheme::default(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("A code editor")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self.process_event(message) {
            Some(tasks) => Task::batch(tasks),
            None => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        if let Some(text_box) = &self.text_box {
            return self.text_box_view(text_box);
        }
        self.no_workspace_view()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // subscription::events()
        // let app_events = subscription::events_with(|event, _status| match event {
        // Event::Keyboard(keyboard::Event::KeyPressed { modifiers, .. }) => {
        //     Message::from_keycode(key_code, Modifiers::from(modifiers))
        // }
        // Event::Keyboard(keyboard::Event::KeyReleased { modifiers, .. }) => {
        //     Message::from_keyrelease(Modifiers::from(modifiers))
        // }
        // Event::Keyboard(keyboard::Event::CharacterReceived(character)) => {
        //     if !character.is_control() {
        //         Some(Message::KeyEvent(KeyEvent::CharacterReceived(character)))
        //     } else {
        //         None
        //     }
        // }
        // Event::Window(window::Event::Resized(_)) => None,
        //     _ => None,
        // });
        // let mut events: Vec<Subscription<Message>> = vec![]; // app_events
        // if let Some(lsp) = self.lsp.as_ref() {
        //     let lsp_events = Subscription::run(connect::connect).map(|event| match event {
        //         connect::Event::Response(value) => {
        //             if let LspResponse::Initialized = value {
        //                 return Message::ClientStart(());
        //             }
        //             Message::LspMessage(value)
        //         }
        //         connect::Event::Ready(mut sender) => {
        //             sender.send(connect::Event::Connect(lsp.new_receiver().clone()));
        //             Message::None
        //         }
        //         connect::Event::Disconnected => Message::CloseClient,
        //         _ => Message::None,
        //     });
        //     events.push(lsp_events)
        // }
        Subscription::batch(vec![
            Subscription::run(laurel_lsp::connect).map(Message::LspMessage)
        ])
    }

    pub fn theme(&self) -> LaurelTheme {
        self.theme.clone()
    }
}

impl Editor {
    fn open(&mut self, file: &str, old_file: Option<Url>, tasks: &mut Vec<Task<Message>>) {
        info!("Opening file {}", file);

        let document = Document::open(file).expect("Couldn't open file");
        self.set_file(document, old_file);
    }

    fn set_file(&mut self, document: Document, old_file: Option<Url>) {
        let document_string = document.to_string();
        let highlighter_config = HighlighterConfig::rust_config(&document_string);

        if let Some(ref mut lsp_connection) = self.lsp {
            let mut lsp_connection = lsp_connection.clone();

            let file_uri = document.uri().clone();

            if let Some(old_file_uri) = old_file {
                lsp_connection.send(LspCommand::Notification(
                    LspServerNotification::Synchronise(Synchronise::DidClose, old_file_uri),
                ));

                lsp_connection.send(LspCommand::Notification(
                    LspServerNotification::Synchronise(
                        Synchronise::DidOpen(document_string),
                        file_uri,
                    ),
                ));
            } else {
                lsp_connection.send(LspCommand::Notification(
                    LspServerNotification::Synchronise(
                        Synchronise::DidOpen(document_string),
                        file_uri,
                    ),
                ));
            }
        }

        let buffer = Buffer::new(document, highlighter_config);
        self.text_box = Some(Textbox::new(buffer).font(Font::MONOSPACE).font_size(14.0));
    }

    fn process_event(&mut self, message: Message) -> Option<Vec<Task<Message>>> {
        let mut tasks: Vec<Task<Message>> = Vec::new();
        match message {
            Message::KeyEvent(event) => {
                self.process_keyboard_event(event, &mut tasks);
            }
            Message::CursorEvent(pos) => {
                let textbox = self.can_edit_textbox()?.set_selection(pos).set_curor(pos);
                textbox.set_floating_message();
                textbox.clear();
            }
            Message::Offset(offset_x, offset_y) => {
                if let Some(textbox) = self.text_box.as_mut() {
                    textbox.set_offset(offset_x, offset_y);
                    textbox.clear();
                }
            }
            Message::View(height, width) => {
                if !self
                    .can_edit_textbox()?
                    .buffer()
                    .window
                    .size_is_different(width, height)
                {
                    self.can_edit_textbox()?.set_size(width, height);
                    self.can_edit_textbox()?.clear();
                }
            }
            // Message for when the user drags to a certain position
            Message::SelectionMove(pos) => {
                self.can_edit_textbox()?
                    .set_selection_end(pos)
                    .set_curor(pos)
                    .clear();
            }
            Message::Paste(value) => {
                self.can_edit_textbox()?.insert(value);
            }
            Message::Open(file) => {
                self.change_file(file, &mut tasks);
            }
            Message::LspNotification(notification) => {
                self.can_edit_textbox()?
                    .process_lsp_notification(notification);
                self.can_edit_textbox()?.clear();
            }
            Message::DismissModal => {
                self.close_floating_elements();
            }
            Message::SelectFile => {
                let file = self.open_file(None);
                if let Some(file) = file {
                    if let Some(file) = file.as_path().to_str() {
                        self.change_file(file.to_owned(), &mut tasks)
                    }
                }
            }
            Message::FileFilter(filter) => self.file_filter = filter,
            Message::Save => {
                // let workspace = self.workspace().to_owned().clone();
                if let Some(textbox) = self.text_box.as_mut() {
                    textbox.save();
                    let file_path = textbox.buffer().document().uri().clone();
                    let mut lsp = self.lsp.clone()?;

                    lsp.send(LspCommand::Notification(
                        LspServerNotification::Synchronise(Synchronise::DidSave(None), file_path),
                    ));
                }
            }
            Message::SetTextBoxFocus(focus) => {
                if let Some(textbox) = self.text_box.as_mut() {
                    textbox.set_focus(focus)
                }
            }
            Message::CloseClient => {}
            Message::ClientStart(_) => {}
            Message::OpenDocument(_) => (),
            Message::CloseDocument(_) => (),
            Message::DocChanged(_) => (),
            Message::HasInitialized(_) => (),
            Message::DidSave(_) => (),
            Message::Done(_) => (),
            Message::None => {}
            Message::LspMessage(m) => match m {
                LspMessage::Initialized(conn) => {
                    debug!(connection = ?conn, "Lsp stream connection initialized");
                    self.lsp = Some(conn);
                }
                LspMessage::Notification(n) => {
                    info!(notification = ?n, "Notification from LSP");
                }
                LspMessage::Response(r) => {
                    info!(response = ?r, "Response from LSP")
                }
                LspMessage::Shutdown => {
                    warn!("Lsp shutdown");
                }
            },

            _ => {}
        }
        if let Some(textbox) = self.text_box.as_mut() {
            textbox.correct_position();
            let window = textbox.buffer().window;
            self.correct_scroll(&mut tasks, window);
        }

        Some(tasks)
    }

    fn no_workspace_view<'a>(&self) -> Element<'a, Message, Renderer> {
        let padding = Padding::new(0.0).bottom(20.0);

        layout::layout(
            container(
                column!(
                    container(text("Welcome to the prototype!")).padding(padding),
                    container(text(
                        "Use the keyboard command ctrl/command+o to open a new folder"
                    ))
                )
                .align_x(iced::alignment::Alignment::Center),
            )
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            self.modal_view(),
            self.is_saved(),
        )
    }

    fn process_keyboard_event(
        &mut self,
        event: KeyEvent,
        _tasks: &mut Vec<Task<Message>>,
    ) -> Option<()> {
        match event {
            // KeyEvent::Special(key, modifiers) => {
            //     self.last_event = key;
            //     self.modifiers = modifiers;
            //     if self.modifiers.ctrl {
            //         dbg!("Ctrl!");
            //         match key {
            //             Key::KeyS => {
            //                 dbg!("S!");
            //                 let workspace = self.workspace().to_owned().clone();
            //                 self.can_edit_textbox()?.save(workspace);
            //                 let file_path =
            //                     self.text_box.as_ref()?.buffer().filename().unwrap().clone();
            //
            //                 // Save command
            //                 let command = Task::perform(
            //                     self.lsp.as_ref()?.as_initialized()?.did_save(file_path),
            //                     Message::DidSave,
            //                 );
            //                 commands.push(command);
            //             }
            //             Key::KeyC => self.can_edit_textbox()?.copy(commands),
            //             Key::KeyV => self.can_edit_textbox()?.paste(commands),
            //             Key::KeyX => self.can_edit_textbox()?.cut(commands),
            //             Key::KeyP => {
            //                 if let Some(path) = &self.workspace {
            //                     if let Some(file) = path.to_str() {
            //                         self.set_modal(file.to_owned());
            //                     }
            //                 }
            //             }
            //             Key::KeyN => self.new_file(commands),
            //             Key::KeyO => self.set_workspace(self.open_folder(), commands),
            //             Key::KeyL => self.can_edit_textbox()?.set_floating_message(),
            //             Key::KeyA => self.can_edit_textbox()?.select_all(commands),
            //             _ => (),
            //         }
            //     }
            //     match key {
            //         Key::KeyUp => self.can_edit_textbox()?.move_up(modifiers),
            //         Key::KeyDown => self.can_edit_textbox()?.move_down(modifiers),
            //         Key::KeyRight => self.can_edit_textbox()?.move_right(modifiers),
            //         Key::KeyLeft => self.can_edit_textbox()?.move_left(modifiers),
            //         Key::KeyDelete => self.can_edit_textbox()?.delete(),
            //         Key::KeyBackSpace => self.can_edit_textbox()?.backspace(),
            //         Key::KeyEnter => self.can_edit_textbox()?.new_line(),
            //         Key::KeyTab => {
            //             self.text_box.as_mut()?.insert(" ".repeat(4).to_owned());
            //             // if let Some(change) = document_change {
            //             //     // self.doc_changed(change);
            //             // }
            //         }
            //         Key::KeyEsc => self.modal = None,
            //         Key::None => (),
            //         Key::KeyHome => self.can_edit_textbox()?.move_start(),
            //         Key::KeyEnd => self.can_edit_textbox()?.move_end(),
            //         Key::KeyPgUp => self.can_edit_textbox()?.page_up(),
            //         Key::KeyPgDown => self.can_edit_textbox()?.page_down(),
            //         _ => (),
            //     }
            // }
            KeyEvent::CharacterReceived(character) => {
                // Only process the event if the last key recieved wasn't a special key.last_event
                // This is because they send the event of special keys twice
                if !self.modifiers.ctrl {
                    self.can_edit_textbox()?.insert(character.to_string());
                }
            }
            _ => {}
        }

        let text_width = self.text_box.as_ref()?.text_width();
        let longest_line = self.text_box.as_ref()?.longest_line();
        self.can_edit_textbox()?
            .correct_position_to_cursor(text_width, longest_line);

        self.text_box.as_ref()?.clear();

        Some(())
    }

    /*
       For methods that requiere editing the state of the textbox.
    */
    fn can_edit_textbox(&mut self) -> Option<&mut Textbox> {
        if let Some(textbox) = self.text_box.as_ref() {
            if textbox.is_focused() {
                return self.text_box.as_mut();
            }
        }
        None
    }

    fn modal_view<'a>(&self) -> Option<Element<'a, Message, Renderer>> {
        self.modal
            .as_ref()
            .map(|value| value.show(&self.file_filter))
    }

    fn is_saved(&self) -> bool {
        if let Some(text_box) = self.text_box.as_ref() {
            return text_box.is_saved();
        }
        true
    }

    // fn set_modal(&mut self, file: String) {
    //     self.modal = Some(Modal::FileSelector(file.to_owned()));
    //     if let Some(textbox) = self.text_box.as_mut() {
    //         textbox.set_focus(false);
    //     }
    // }
    //
    // fn open_folder(&self) -> Option<PathBuf> {
    //     let file = FileDialog::new()
    //         // .set_directory("/")
    //         .pick_folder();
    //     if let Some(file) = file {
    //         return Some(file);
    //     };
    //     None
    // }

    fn open_file(&self, path: Option<String>) -> Option<PathBuf> {
        let mut dialog = FileDialog::new();
        if let Some(path) = path {
            dialog = dialog.set_directory(path);
        }
        let file = dialog.pick_file();
        if let Some(file) = file {
            return Some(file);
        };
        None
    }

    fn change_file(&mut self, file: String, tasks: &mut Vec<Task<Message>>) {
        let old_file = self
            .text_box
            .as_ref()
            .map(Textbox::buffer)
            .map(Buffer::document)
            .map(Document::uri)
            .map(ToOwned::to_owned);

        self.open(&file, old_file, tasks);
        self.modal = None;
    }

    // fn workspace(&self) -> Option<String> {
    //     if let Some(value) = self.workspace.as_ref() {
    //         return value.to_str().map(|value| value.to_string());
    //     }
    //     None
    // }

    // fn set_workspace(&mut self, file: Option<PathBuf>, commands: &mut Vec<Task<Message>>) {
    //     self.workspace = file;
    //     if let Some(file) = &self.workspace {
    //         let lsp = LspConnection::new(file);
    //         let client = match lsp {
    //             Ok(lsp) => {
    //                 let init_params = lsp.init_params();
    //                 if let LspClient::Uninitialized(sender) = lsp.new_sender() {
    //                     let fut = sender.initialize(init_params);
    //                     commands.push(Task::perform(fut, Message::Done));
    //                 }
    //                 Some(lsp)
    //             }
    //             Err(_e) => {
    //                 eprintln!("Failed to initialize the client");
    //                 None
    //             }
    //         };
    //         self.client_id += 1;
    //         self.lsp = client;
    //         if self.text_box.is_none() {
    //             self.set_modal(file.to_str().unwrap().to_owned());
    //         }
    //     }
    //     if self.text_box.is_none() {
    //         self.new_file(commands);
    //     }
    // }

    fn text_box_view<'a>(&'a self, text_box: &'a Textbox) -> Element<'a, Message, Renderer> {
        let id = iced::widget::scrollable::Id::new("1");
        let scroll_properties = Scrollbar::default();

        let second_scroll_id = iced::widget::scrollable::Id::new("2");
        container(layout::layout(
            row![
                line_number(
                    text_box.buffer().len(),
                    text_box.get_font_size(),
                    text_box.height() + text_box.window_height(),
                    second_scroll_id
                ),
                ViewPort::new(
                    scrollable(TextboxContainer::new(
                        text_box.view(),
                        text_box,
                        text_box.longest_line(),
                        text_box.floating_element(),
                        text_box.get_font_size(),
                        text_box.get_font()
                    ))
                    .id(id)
                    .on_scroll(|viewport| {
                        Message::Offset(viewport.absolute_offset().x, viewport.absolute_offset().y)
                    })
                    .direction(scrollable::Direction::Both {
                        vertical: scroll_properties,
                        horizontal: scroll_properties
                    })
                    .into(),
                    self.modal_view(),
                    &text_box.buffer().window,
                )
            ]
            .spacing(5)
            .into(),
            self.modal_view(),
            self.is_saved(),
        ))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }

    // fn no_file_view<'a>(&self) -> Element<'a, Message, Renderer> {
    //     layout::layout(
    //         container(container(text("Use 'Ctrl+p' to select and open a file")))
    //             .align_x(iced::alignment::Horizontal::Center)
    //             .align_y(iced::alignment::Vertical::Center)
    //             .width(Length::Fill)
    //             .height(Length::Fill)
    //             .into(),
    //         self.modal_view(),
    //         self.is_saved(),
    //     )
    // }

    fn correct_scroll(&self, tasks: &mut Vec<Task<Message>>, window: VirtualWindow) {
        let id = iced::widget::scrollable::Id::new("1");
        let scroll_command: Task<Message> = iced::widget::scrollable::scroll_to(id, window.into());
        tasks.push(scroll_command);

        let id = iced::widget::scrollable::Id::new("2");
        let scroll_command: Task<Message> = iced::widget::scrollable::scroll_to(id, window.into());
        tasks.push(scroll_command);
    }

    /**
     * Closes both floating elements and modals
     */
    fn close_floating_elements(&mut self) {
        if let Some(text_box) = self.can_edit_textbox() {
            text_box.clear_floating_elements();
        }
        self.modal = None;
    }
}
