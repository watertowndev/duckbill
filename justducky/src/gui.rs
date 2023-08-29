use std::path::PathBuf;
use duckbill::duckfile::duckerror::DuckError;
use iced::{Element, Error, Font, Sandbox, Settings};
use iced::widget::{Button, Text, Column, Container, Row, TextInput, text_input};
use native_dialog::{FileDialog, MessageDialog};
use duckbill::duckfile::duckacctid::DuckAcctId;
use crate::applogic::{AppLogic, AppState};


#[derive(Debug, Clone )]
pub enum AppStateMsg {
    PickFile,
    LoadFile,

    ChopToEnd,
    ChopFromStart,
    ChopRange,
    ChopOne,

    InputAChanged(String),
    InputBChanged(String),

    Chop,

    ErrorMessage(String),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputStatus {
    Empty,
    Incomplete,
    Invalid,
    ValidNotFound,
    ValidFound,
}

struct AppUIState {
    picked_file: Option<PathBuf>,
    chop_message: Option<AppStateMsg>,
    bill_input_a: String,
    bill_input_b: String,
    app_logic_state: AppLogic,
    bill_input_a_status: InputStatus,
    bill_input_b_status: InputStatus,
}

impl Sandbox for AppUIState {
    type Message = AppStateMsg;

    fn new() -> Self {
        AppUIState {
            picked_file: None,
            chop_message: None,
            bill_input_a: String::new(),
            bill_input_b: String::new(),
            bill_input_a_status: InputStatus::Empty,
            bill_input_b_status: InputStatus::Empty,
            app_logic_state: AppLogic::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Just Ducky")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppStateMsg::PickFile => {
                self.pick_file();
                if self.picked_file.is_some() {
                    self.update(AppStateMsg::LoadFile)
                }
            }

            AppStateMsg::LoadFile => {
                match self.load_file() {
                    Ok(_) => {}
                    Err(_) => self.update(AppStateMsg::ErrorMessage(String::from("Unable to load file.")))
                }
            }

            AppStateMsg::Chop => {
                match (&self.bill_input_a_status, &self.bill_input_b_status) {
                    (InputStatus::ValidFound, InputStatus::ValidFound) => {
                        println!("Chop range or single");
                    }
                    (InputStatus::ValidFound, InputStatus::Empty) => {
                        println!("Chop to end");
                    }
                    (InputStatus::Empty, InputStatus::ValidFound) => {
                        println!("Chop from start");
                    }
                    (_, _) => {
                        println!("Something else?");
                    }
                }
                //MessageDialog::new().set_title("Ducked")
                //    .set_text(&format!("{:?}", (&self.bill_input_a_status, &self.bill_input_b_status) ))
                //    .show_alert();
            }

            AppStateMsg::InputAChanged(s) => {
                if self.picked_file.is_some() {
                    self.bill_input_a = s;
                    self.chop_message = None;
                    match self.acct_id_is_valid(&self.bill_input_a) {
                        Ok(m) => {
                            self.bill_input_a_status = m;
                        }
                        Err(m) => {
                            self.bill_input_a_status = m;
                        }
                    }
                    self.update_duck_it_button();
                }
            }
            AppStateMsg::InputBChanged(s) => {
                if self.picked_file.is_some() {
                    self.bill_input_b = s;
                    self.chop_message = None;
                    match self.acct_id_is_valid(&self.bill_input_b) {
                        Ok(m) => {
                            self.bill_input_b_status = m;
                        }
                        Err(m) => {
                            self.bill_input_b_status = m;
                        }
                    }
                    self.update_duck_it_button();
                }
            }

            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header = Text::new("Choose an option");
        let button_pickfile = Button::new("Choose file...").width(250).on_press(AppStateMsg::PickFile);
        let current_filename = match &self.picked_file {
            None => {"No file selected.".to_string()}
            Some(f) => {f.to_string_lossy().to_string()}
        };
        let current_file_text = Text::new(current_filename);
        let file_pick_row = Row::new()
            .push(button_pickfile)
            .push(current_file_text);

        let bill_id_a = TextInput::new("Bill A", &self.bill_input_a).width(250).size(30)
            .on_input(AppStateMsg::InputAChanged)
            .icon(text_input::Icon {
                font: Font::default(),
                code_point: AppUIState::get_icon(&self.bill_input_a_status),
                size: Some(24.0),
                spacing: 10.0,
                side: text_input::Side::Right,
            });
        let bill_id_b = TextInput::new("Bill B", &self.bill_input_b).width(250).size(30)
            .on_input(AppStateMsg::InputBChanged)
            .icon(text_input::Icon {
                font: Font::default(),
                code_point: AppUIState::get_icon(&self.bill_input_b_status),
                size: Some(24.0),
                spacing: 10.0,
                side: text_input::Side::Right,
            });
        let button_duck_it = Button::new("Duck It!").width(250).on_press_maybe(self.chop_message.clone());

        let bill_action_inputs = Row::new()
            .push(bill_id_a)
            .push(bill_id_b)
            .spacing(20);

        let col = Column::new()
            .push(header)
            .push(file_pick_row)
            .push(bill_action_inputs)
            .push(button_duck_it)
            .spacing(20);

        Container::new(col).center_x().center_y().width(1000).height(300).into()
    }
}
impl AppUIState {
    pub fn pick_file(&mut self) {
        let dialog_sel = FileDialog::new()
            .set_location("~")
            .show_open_single_file();
        if let Ok(maybe_path) = dialog_sel {
            if maybe_path.is_some() { // None means cancel was pressed
                self.picked_file = maybe_path;
            }
        }
    }
    pub fn load_file(&mut self) -> Result<(),()>{
        if let Some(picked_f) = self.picked_file.clone() {
            return match self.app_logic_state.load_file_pathbuf(&picked_f) {
                Ok(_) => {
                    println!("loadfile ok");
                    Ok(())
                }
                Err(_) => {
                    println!("loadfile err");
                    self.picked_file = None;
                    Err(())
                }
            }
        }
        Ok(())
    }
    pub fn acct_id_is_valid(&self, acctid: &str) -> Result<InputStatus, InputStatus> {
        let id_bytes = acctid.trim().as_bytes().to_owned();
        if id_bytes.len() == 0 {
            return Err(InputStatus::Empty);
        }
        if id_bytes.len() < 12 {
            return Err(InputStatus::Incomplete);
        }
        match DuckAcctId::try_from(id_bytes) {
            Ok(id) => {
                match self.app_logic_state.original_bills.get_index_of_account(&id) {
                    None => Err(InputStatus::ValidNotFound),
                    Some(_) => Ok(InputStatus::ValidFound)
                }
            }
            Err(_) => Err(InputStatus::Invalid)
        }
    }

    pub fn get_icon(i: &InputStatus) -> char {
        match i {
            InputStatus::Empty => ' ',
            InputStatus::Incomplete => '⌨',
            InputStatus::Invalid => '❌',
            InputStatus::ValidNotFound => '⭕',
            InputStatus::ValidFound => '✅'
        }
    }

    pub fn update_duck_it_button(&mut self) {
        self.chop_message = match (&self.bill_input_a_status, &self.bill_input_b_status) {
            (InputStatus::ValidFound, InputStatus::ValidFound) => {
                Some(AppStateMsg::Chop)
            }
            (InputStatus::ValidFound, InputStatus::Empty) => {
                Some(AppStateMsg::Chop)
            }
            (InputStatus::Empty, InputStatus::ValidFound) => {
                Some(AppStateMsg::Chop)
            }
            (_, _) => {
                None
            }
        }
    }
    pub fn disable_bill_actions(&mut self) {
    }
}

pub fn gui() -> Result<(), Error> {
    AppUIState::run(Settings::default())
}
/*
println!("Use the dialog to select a file.");
let dialog_sel = FileDialog::new()
.set_location("~")
.show_open_single_file();
if let Ok(maybe_path) = dialog_sel {
file_choice = maybe_path;
}
else {
println!("No filename given, returning to menu.");


 */