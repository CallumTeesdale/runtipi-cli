use ratatui::backend::Backend;

pub struct App {
    pub name: String,
    pub should_quit: bool,
}
impl App {
    pub fn new() -> Self {
        Self {
            name: "Runtipi".to_string(),
            should_quit: false,
        }
    }
    pub fn tick(&self) {}
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
