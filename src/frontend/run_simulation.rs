use super::FrontEnd;

impl FrontEnd {
    pub(super) fn run_simulation(&mut self) {
        if !self.check_valid_state() {
            self.status_msg = String::from("A goal and a car must be in the grid");
            return;
        }
        if !self.grid.find_path() {
            self.status_msg = String::from("Couldn't find a path");
        } else {
            self.status_msg = String::from("Path found!");
        }
    }

    fn check_valid_state(&self) -> bool {
        self.grid.has_car() && self.grid.has_goal()
    }
}
