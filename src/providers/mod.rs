#[derive(Clone, Default, Debug)]
pub struct LoggedIn(pub bool);

impl LoggedIn {
    pub fn set(&mut self, value: bool) {
        self.0 = value;
    }
}
