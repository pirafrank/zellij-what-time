use crate::configuration::Configuration;

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
    pub fn render(&self, config: &Configuration) -> String {
        let mut output = String::new();
        if config.get_has_date() {
            output.push_str(&config.get_separator());
            output.push_str(&self.date);
        }
        if config.get_has_time() {
            output.push_str(&config.get_separator());
            output.push_str(&self.time);
        }
        output
    }
}
