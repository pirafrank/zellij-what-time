mod configuration;
mod date_time;

use std::collections::BTreeMap;
use chrono::Utc;
use configuration::Configuration;
use zellij_tile::prelude::*;

fn format_terminal_style(text: &str, fg_color: &str, bg_color: &str) -> String {
    format!("\x1b[{};{}m{}\x1b[0m", fg_color, bg_color, text)
}

static PLUGIN_NAME: &str = "zellij-what-time";
static DATE_CMD: &str = "date";

#[derive(Default)]
struct State {
    output: String,
    date_cmd_args: String,
    last_update: f64,
    config: Configuration,
    has_permissions: bool,
}

register_plugin!(State);

// zellij plugin development materials
// docs : https://zellij.dev/docs/overview
// guide: https://web.archive.org/web/20240903124717/https://blog.nerd.rocks/posts/common-snippets-for-zellij-development/

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        // load user configuration
        self.config.load_user_config(&configuration);
        // set up logging only if user has specified a log level
        if self.config.is_log_enabled() {
            init_tracing(&self.config.get_log_level());
            tracing::info!("Configuration: {:?}", self.config.to_string());
        }
        // init permissions status
        // note: Zellij will grant permissions after user accepts the prompt on first run.
        //       All subsequent runs will have PermissionRequestResult event anyway with
        //       a granted status (if all went ok).
        self.has_permissions = false;
        // initialize last_update to 0 to force an update on the first render
        self.last_update = 0.0;
        // init date command
        self.init_datetime_cmd();
        request_permission(&[
            PermissionType::RunCommands,
        ]);
        subscribe(&[
            EventType::Timer,
            EventType::PermissionRequestResult,
            EventType::RunCommandResult,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        let timeout_interval = self.config.get_interval_update();
        match event {
            Event::PermissionRequestResult(result ) => {
                if result == PermissionStatus::Granted {
                    tracing::debug!("Permission granted!");
                    self.has_permissions = true;
                } else {
                    tracing::error!("Permission denied. PermissionStatus: {:?}", result);
                }
                // no matter what, this plugin is a status-bar plugin
                // so it is not selectable anyway.
                set_selectable(false);
                // after getting permissions, do not wait for the timer to render.
                // this avoids at zellij start an empty render until the first timer event.
                tracing::debug!("Just loaded, exec first update");
                self.refresh_last_update();
                self.run_datetime_cmd();
                set_timeout(timeout_interval);
            }
            Event::Timer(_time) => {
                tracing::debug!("Event Timer received, time: {}", _time);
                // now
                let now = get_current_timestamp();
                // Update timezone every minute
                if now - self.last_update >= timeout_interval {
                    tracing::debug!("Time to update");
                    self.refresh_last_update();
                    self.run_datetime_cmd();
                } else {
                    tracing::debug!("Too soon, won't update!");
                }
                set_timeout(timeout_interval);
            }
            // RunCommandResult brings you exit_code, STDOUT, STDERR, context
            Event::RunCommandResult(exit_code, stdout, stderr, _ctx) => {
                tracing::debug!("Got a RunCommandResult event!");
                if exit_code.unwrap() == 0 {
                    let output = String::from_utf8_lossy(&stdout).trim().to_string();
                    tracing::debug!("RunCommandResult: {:?}", output);
                    let date_time = date_time::DateTime::parse(&output);
                    self.output = date_time.render(&self.config);
                } else {
                  let error = String::from_utf8_lossy(&stderr).trim().to_string();
                  tracing::error!("RunCommandResult: {:?}", error);
                  self.output = "Plugin Error".to_string();
                };
                // error or not, render anyway to notify the user
                should_render = true;
            }
            _ => {}
        }
        should_render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        let s: &usize = &self.output.chars()
            .map(|c| if c.is_ascii() { 1 } else { 2 })
            .sum::<usize>();
        let size: usize = *s as usize;
        // because of https://bit.ly/3MYjOlv
        let padding: String = if cols - size > 0 {
            " ".repeat(cols - size)
        } else {
            String::new()
        };
        // TODO: support only right align for now
        // TODO: add actual support for text styles
        let styled_output = format_terminal_style(&self.output, "30", "37");
        let to_render: String = format!("{}{}", padding, styled_output);

        tracing::debug!("Render: output: {}", to_render);
        print!("{}", to_render);
    }
}

impl State {
    fn refresh_last_update(&mut self) {
        self.last_update = get_current_timestamp();
    }
    fn init_datetime_cmd(&mut self) {
        let mut date_args: String = String::new();
        date_args.push_str("+");  // date command format
        date_args.push_str(&self.config.get_date_format());
        date_args.push_str(&date_time::DATE_ARG_SEP);
        date_args.push_str(&self.config.get_time_format());
        self.date_cmd_args = date_args;
    }
    fn run_datetime_cmd(&mut self) {
        tracing::debug!("Fired run_datetime_cmd");
        let date_cmd: [&str; 2] = [DATE_CMD, &self.date_cmd_args];
        zellij_tile::shim::run_command(&date_cmd, BTreeMap::new());
    }
}

fn get_current_timestamp() -> f64 {
    Utc::now().timestamp() as f64
}

fn init_tracing(level: &str) {
    use std::fs::File;
    use std::sync::Arc;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::fmt;
    use tracing_subscriber::filter;

    let filter = filter::EnvFilter::new(format!("{}={}", PLUGIN_NAME, level))
        .add_directive(level.parse().unwrap());

    let file = File::create(format!(".{}.log", PLUGIN_NAME));
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = fmt::layer().with_writer(Arc::new(file));
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(debug_log);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    tracing::info!("Logging initialized");
}
