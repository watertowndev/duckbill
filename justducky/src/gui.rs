use std::path::PathBuf;
use duckbill::duckfile::duckerror::DuckError;
use iced::{Element, Error, Sandbox, Settings};
use iced::widget::{Button, Text, Column, Container, Row, TextInput};
use native_dialog::FileDialog;
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

    ErrorMessage(String),
}

struct AppUIState {
    picked_file: Option<PathBuf>,
    chop_to_end_message: Option<AppStateMsg>,
    chop_from_start_message: Option<AppStateMsg>,
    chop_range_message: Option<AppStateMsg>,
    chop_one_message: Option<AppStateMsg>,
    bill_input_a: String,
    bill_input_b: String,
    app_logic_state: AppLogic,
}

impl Sandbox for AppUIState {
    type Message = AppStateMsg;

    fn new() -> Self {
        AppUIState {
            picked_file: None,
            chop_to_end_message: None,
            chop_from_start_message: None,
            chop_range_message: None,
            chop_one_message: None,
            bill_input_a: String::new(),
            bill_input_b: String::new(),
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
                if let Some(picked_f) = self.picked_file.clone() {
                    match self.app_logic_state.load_file_pathbuf(&picked_f) {
                        Ok(_) => {
                            self.enable_bill_actions();
                        }
                        Err(_) => {
                            self.disable_buttons();
                            self.picked_file = None;
                            self.update(AppStateMsg::ErrorMessage(String::from("Unable to load file.")));
                        }
                    }
                }
                if self.picked_file.is_some() {
                    self.chop_to_end_message = Some(AppStateMsg::ChopToEnd);
                }
                else {
                    self.chop_to_end_message = None;
                }
            }

            AppStateMsg::InputAChanged(s) => {
                println!("{}", s);
                self.bill_input_a = s;
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        println!("view!");
        let header = Text::new("Choose an option");
        let button_pickfile = Button::new("Choose file...").width(250).on_press(AppStateMsg::PickFile);
        let button_chop_to_end = Button::new("BILL to End").width(200).on_press_maybe(self.chop_to_end_message.clone());
        let button_chop_from_start = Button::new("Start to BILL").width(200).on_press_maybe(self.chop_from_start_message.clone());
        let button_chop_range = Button::new("BILL A to BILL B").width(200).on_press_maybe(self.chop_range_message.clone());
        let button_chop_one = Button::new("Single BILL").width(200).on_press_maybe(self.chop_one_message.clone());
        let current_file = Text::new(format!("{:?}", self.picked_file.clone().unwrap_or(PathBuf::new()).to_str().unwrap_or("No File Selected")) );
        let bill_action_buttons = Row::new()
            .push(button_chop_to_end)
            .push(button_chop_from_start)
            .push(button_chop_range)
            .push(button_chop_one)
            .spacing(20);
        let bill_id_a = TextInput::new("Bill A", &self.bill_input_a).width(250).on_input(AppStateMsg::InputAChanged);
        let bill_id_b = TextInput::new("Bill B", &self.bill_input_b).width(250).on_input(AppStateMsg::InputBChanged);
        let bill_action_inputs = Row::new()
            .push(bill_id_a)
            .push(bill_id_b)
            .spacing(20);

        let col = Column::new()
            .push(header)
            .push(button_pickfile)
            .push(current_file)
            .push(bill_action_buttons)
            .push(bill_action_inputs)
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
            self.picked_file = maybe_path;
        }
    }

    pub fn enable_bill_actions(&mut self) {
        self.chop_to_end_message=Some(AppStateMsg::ChopToEnd);
        self.chop_from_start_message=Some(AppStateMsg::ChopFromStart);
        self.chop_range_message=Some(AppStateMsg::ChopRange);
        self.chop_one_message=Some(AppStateMsg::ChopOne);
    }
    pub fn disable_buttons(&mut self) {
        self.chop_to_end_message=None;
        self.chop_from_start_message=None;
        self.chop_range_message=None;
        self.chop_one_message=None;
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