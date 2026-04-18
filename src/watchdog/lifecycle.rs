#[derive(Clone, Debug, Default, PartialEq)]
pub struct IncidentState {
    pub rounds_written: u32,
    pub active: bool,
}

impl IncidentState {
    pub fn new() -> Self {
        Self {
            rounds_written: 0,
            active: false,
        }
    }

    pub fn should_continue(&self, max_rounds: u32) -> bool {
        self.active && self.rounds_written < max_rounds
    }
}
