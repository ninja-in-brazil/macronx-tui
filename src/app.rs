use crossterm::event::KeyCode;

use crate::api::ApiClient;
use crate::config::Config;
use crate::models::{CreateInboxBody, CreateInboxRequest, Inbox};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    InboxList,
    InboxShow,
    InboxCreate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormField {
    Name,
    Source,
    Summary,
    Payload,
    Metadata,
}

impl FormField {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Source => "Source",
            Self::Summary => "Summary",
            Self::Payload => "Payload (JSON)",
            Self::Metadata => "Metadata (JSON)",
        }
    }

    pub fn all() -> [FormField; 5] {
        [
            FormField::Name,
            FormField::Source,
            FormField::Summary,
            FormField::Payload,
            FormField::Metadata,
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreateForm {
    pub name: String,
    pub source: String,
    pub summary: String,
    pub payload: String,
    pub metadata: String,
    pub active_field: usize,
}

impl CreateForm {
    pub fn active_value_mut(&mut self) -> &mut String {
        match self.active_field {
            0 => &mut self.name,
            1 => &mut self.source,
            2 => &mut self.summary,
            3 => &mut self.payload,
            4 => &mut self.metadata,
            _ => unreachable!(),
        }
    }

    pub fn value_for(&self, field: &FormField) -> &str {
        match field {
            FormField::Name => &self.name,
            FormField::Source => &self.source,
            FormField::Summary => &self.summary,
            FormField::Payload => &self.payload,
            FormField::Metadata => &self.metadata,
        }
    }

    pub fn next_field(&mut self) {
        self.active_field = (self.active_field + 1) % 5;
    }

    pub fn prev_field(&mut self) {
        self.active_field = self.active_field.checked_sub(1).unwrap_or(4);
    }

    pub fn clear(&mut self) {
        *self = CreateForm::default();
    }
}

pub struct App {
    pub screen: Screen,
    pub inboxes: Vec<Inbox>,
    pub selected: usize,
    pub current_inbox: Option<Inbox>,
    pub form: CreateForm,
    pub status: Option<(String, bool)>, // (message, is_error)
    pub should_quit: bool,
    pub client: ApiClient,
}

impl App {
    pub fn new(config: Config) -> Self {
        let client = ApiClient::new(&config.base_url, &config.api_token);
        Self {
            screen: Screen::InboxList,
            inboxes: Vec::new(),
            selected: 0,
            current_inbox: None,
            form: CreateForm::default(),
            status: None,
            should_quit: false,
            client,
        }
    }

    pub fn load_inboxes(&mut self) {
        match self.client.list_inboxes() {
            Ok(inboxes) => {
                self.inboxes = inboxes;
                if self.selected >= self.inboxes.len() && !self.inboxes.is_empty() {
                    self.selected = self.inboxes.len() - 1;
                }
                self.status = None;
            }
            Err(e) => {
                self.status = Some((format!("Error loading inboxes: {}", e), true));
            }
        }
    }

    pub fn open_selected_inbox(&mut self) {
        if self.inboxes.is_empty() {
            return;
        }
        let id = self.inboxes[self.selected].id;
        match self.client.get_inbox(id) {
            Ok(inbox) => {
                self.current_inbox = Some(inbox);
                self.screen = Screen::InboxShow;
                self.status = None;
            }
            Err(e) => {
                self.status = Some((format!("Error loading inbox: {}", e), true));
            }
        }
    }

    pub fn submit_create_form(&mut self) {
        let payload = match serde_json::from_str::<serde_json::Value>(&self.form.payload) {
            Ok(v) => v,
            Err(_) => {
                if self.form.payload.trim().is_empty() {
                    serde_json::Value::Object(Default::default())
                } else {
                    self.status = Some(("Payload is not valid JSON".to_string(), true));
                    return;
                }
            }
        };
        let metadata = match serde_json::from_str::<serde_json::Value>(&self.form.metadata) {
            Ok(v) => v,
            Err(_) => {
                if self.form.metadata.trim().is_empty() {
                    serde_json::Value::Object(Default::default())
                } else {
                    self.status = Some(("Metadata is not valid JSON".to_string(), true));
                    return;
                }
            }
        };

        let req = CreateInboxRequest {
            inbox: CreateInboxBody {
                name: self.form.name.clone(),
                source: self.form.source.clone(),
                summary: self.form.summary.clone(),
                payload,
                metadata,
            },
        };

        match self.client.create_inbox(req) {
            Ok(inbox) => {
                self.form.clear();
                self.current_inbox = Some(inbox);
                self.screen = Screen::InboxShow;
                self.status = Some(("Inbox created successfully".to_string(), false));
                self.load_inboxes();
            }
            Err(e) => {
                self.status = Some((format!("Error creating inbox: {}", e), true));
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        // Clear non-error status on any keypress
        if let Some((_, false)) = &self.status {
            self.status = None;
        }

        match self.screen {
            Screen::InboxList => self.handle_list_key(key),
            Screen::InboxShow => self.handle_show_key(key),
            Screen::InboxCreate => self.handle_create_key(key),
        }
    }

    fn handle_list_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.inboxes.is_empty() {
                    self.selected = (self.selected + 1).min(self.inboxes.len() - 1);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
            }
            KeyCode::Char('g') => {
                self.selected = 0;
            }
            KeyCode::Char('G') => {
                if !self.inboxes.is_empty() {
                    self.selected = self.inboxes.len() - 1;
                }
            }
            KeyCode::Enter => {
                self.open_selected_inbox();
            }
            KeyCode::Char('n') => {
                self.form.clear();
                self.screen = Screen::InboxCreate;
                self.status = None;
            }
            KeyCode::Char('r') => {
                self.load_inboxes();
            }
            _ => {}
        }
    }

    fn handle_show_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Backspace => {
                self.screen = Screen::InboxList;
                self.status = None;
            }
            _ => {}
        }
    }

    fn handle_create_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.form.clear();
                self.screen = Screen::InboxList;
                self.status = None;
            }
            KeyCode::Tab => {
                self.form.next_field();
            }
            KeyCode::BackTab => {
                self.form.prev_field();
            }
            KeyCode::Enter => {
                self.submit_create_form();
            }
            KeyCode::Backspace => {
                self.form.active_value_mut().pop();
            }
            KeyCode::Char(c) => {
                self.form.active_value_mut().push(c);
            }
            _ => {}
        }
    }
}
