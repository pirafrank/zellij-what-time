pub static DATE_ARG_SEP: &str = "X";

pub struct DateTime {
    date: String,
    time: String,
}

impl Default for DateTime {
    fn default() -> Self {
        Self {
            date: String::new(),
            time: String::new(),
        }
    }
}

impl DateTime {
    pub fn new(date: String, time: String) -> Self {
        Self { date, time }
    }
    pub fn parse(output: &String) -> Self {
        let mut date = String::new();
        let mut time = String::new();
        let mut iter = output.split(DATE_ARG_SEP);
        date.push_str(iter.next().unwrap());
        time.push_str(iter.next().unwrap());
        Self::new(date, time)
    }
    pub fn render(&self, sep: &String) -> String {
        format!("{}{}{}{}", sep, self.date, sep, self.time)
    }
}
