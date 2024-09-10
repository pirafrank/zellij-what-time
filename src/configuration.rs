use std::collections::BTreeMap;

static DEFAULT_DATE_FORMAT: &str = "%Y.%m.%d %a";
static DEFAULT_TIME_FORMAT: &str = "%H:%M";
static DEFAULT_SEPARATOR: &str = " 〈";
static DEFAULT_INTERVAL_UPDATE: f64 = 60.0;
static DEFAULT_LOG_LEVEL: &str = "debug";

pub struct Configuration {
    date_format: String,
    has_date: bool,
    time_format: String,
    has_time: bool,
    separator: String,
    interval_update: f64,
    log_enabled: bool,
    log_level: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            date_format: DEFAULT_DATE_FORMAT.to_string(),
            has_date: true,
            time_format: DEFAULT_TIME_FORMAT.to_string(),
            has_time: true,
            separator: DEFAULT_SEPARATOR.to_string(),
            interval_update: DEFAULT_INTERVAL_UPDATE,
            log_enabled: false,
            log_level: DEFAULT_LOG_LEVEL.to_string(),
        }
    }
}

impl Configuration {
    pub fn get_date_format(&self) -> &String {
        &self.date_format
    }
    pub fn get_has_date(&self) -> bool {
        self.has_date
    }
    pub fn get_time_format(&self) -> &String {
        &self.time_format
    }
    pub fn get_has_time(&self) -> bool {
        self.has_time
    }
    pub fn get_separator(&self) -> &String {
        &self.separator
    }
    pub fn get_interval_update(&self) -> f64 {
        self.interval_update
    }
    pub fn is_log_enabled(&self) -> bool {
        self.log_enabled
    }
    pub fn get_log_level(&self) -> &String {
        &self.log_level
    }
    pub fn load_user_config(&mut self, configuration: &BTreeMap<String, String>) {
        if let Some(date_format) = configuration.get("date_format") {
            self.has_date = !date_format.is_empty();
            // empty date_format means disabling date
            self.date_format = if self.has_date {
                date_format.to_string()
            } else {
                DEFAULT_DATE_FORMAT.to_string()
            };
        } else {
            // no date_format choice, means using default
            self.has_date = true;
            self.date_format = DEFAULT_DATE_FORMAT.to_string();
        }

        if let Some(time_format) = configuration.get("time_format") {
            self.has_time = !time_format.is_empty();
            // empty time_format means disabling time
            self.time_format = if self.has_time {
                time_format.to_string()
            } else {
                DEFAULT_TIME_FORMAT.to_string()
            };
        } else {
            // no time_format choice, means using default
            self.has_time = true;
            self.time_format = DEFAULT_TIME_FORMAT.to_string();
        }

        if let Some(separator) = configuration.get("separator") {
            self.separator = separator.to_string();
        } else {
            self.separator = DEFAULT_SEPARATOR.to_string();
        }

        if let Some(interval_update) = configuration.get("interval_update") {
            self.interval_update = interval_update.parse::<f64>().unwrap();
        } else {
            self.interval_update = DEFAULT_INTERVAL_UPDATE;
        }

        if let Some(log_level) = configuration.get("log_level") {
            self.log_enabled = !log_level.is_empty();
            self.log_level = if self.log_enabled {
                log_level.to_string()
            } else {
                DEFAULT_LOG_LEVEL.to_string()
            };
        } else {
            // empty log_level means no logging
            self.log_enabled = false;
            self.log_level = DEFAULT_LOG_LEVEL.to_string();
        }

    }
    pub fn to_string(&self) -> String {
        format!(
            "Configuration: date_format: {}, time_format: {}, separator: {}, interval_update: {}, log_level: {}",
            self.date_format, self.time_format, self.separator, self.interval_update, self.log_level
        )
    }
}
