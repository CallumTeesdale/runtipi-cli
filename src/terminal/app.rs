use ratatui::widgets::ListState;


pub struct App {
    pub name: String,
    pub should_quit: bool,
    pub list_state: ListState,
}
impl App {
    pub fn new() -> Self {
        Self {
            name: "Runtipi".to_string(),
            should_quit: false,
            list_state: ListState::default(),
        }
    }
    pub fn tick(&self) {}
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
