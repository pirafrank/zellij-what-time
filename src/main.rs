use std::collections::BTreeMap;
use chrono::Utc;
use tracing_subscriber::util::SubscriberInitExt;
use zellij_tile::prelude::*;

static TIMEOUT_INTERVAL: f64 = 2.0;
static PLUGIN_NAME: &str = "zellij-what-time";

#[derive(Default)]
struct State {
    output: String,
    last_update: f64,
    user_config: BTreeMap<String, String>,
    has_permissions: bool,
}

register_plugin!(State);

// zellij plugin development materials
// docs : https://zellij.dev/docs/overview
// guide: https://web.archive.org/web/20240903124717/https://blog.nerd.rocks/posts/common-snippets-for-zellij-development/

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        init_tracing();
        self.user_config = configuration;
        self.has_permissions = false;
        // initialize last_update to 0 to force an update on the first render
        self.last_update = 0.0;
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
        match event {
            Event::PermissionRequestResult(result ) => {
                if result == PermissionStatus::Granted {
                  self.has_permissions = true;
                }
                // no matter what, this plugin is a status-bar plugin
                // so it is not selectable anyway.
                set_selectable(false);
                set_timeout(TIMEOUT_INTERVAL);
            }
            Event::Timer(_time) => {
                tracing::debug!("Event Timer received, time: {}", _time);
                // now
                let now = get_current_timestamp();
                // Update timezone every minute
                if now - self.last_update >= TIMEOUT_INTERVAL {
                    tracing::debug!("Time to update");
                    self.refresh_last_update();
                    self.run_datetime_cmd();
                } else {
                    tracing::debug!("Too soon, won't update!");
                }
                set_timeout(TIMEOUT_INTERVAL);
            }
            // exit_code, STDOUT, STDERR, context
            Event::RunCommandResult(exit_code, stdout, stderr, _ctx) => {
                tracing::debug!("Got a RunCommandResult event!");
                if exit_code.unwrap() == 0 {
                  let output = String::from_utf8_lossy(&stdout).trim().to_string();
                  tracing::debug!("RunCommandResult: {:?}", output);
                  self.output = output;
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
        let right_padding_size = 1;
        let right_padding = " ".repeat(right_padding_size);
        let size = self.output.len() + right_padding_size;
        // because of https://bit.ly/3MYjOlv
        let padding: String = if cols as isize - size as isize > 0 {
            " ".repeat(cols - size)
        } else {
            String::new()
        };
        // TODO: support only right align for now
        let to_render: String = format!("{}{}{}", padding, self.output, right_padding);
        tracing::debug!("Render: output: {}", to_render);
        print!("{}", to_render);
    }
}

impl State {
    fn refresh_last_update(&mut self) {
        self.last_update = get_current_timestamp();
    }
    fn run_datetime_cmd(&mut self) {
        tracing::debug!("Fired run_datetime_cmd");
        let cmd_w_args = ["date", "+%Y/%m/%d %H:%M:%S"];
        zellij_tile::shim::run_command(&cmd_w_args, self.user_config.clone());
    }
}

fn get_current_timestamp() -> f64 {
    Utc::now().timestamp() as f64
}

fn init_tracing() {
    use std::fs::File;
    use std::sync::Arc;
    use tracing_subscriber::layer::SubscriberExt;

    let file = File::create(format!(".{}.log", PLUGIN_NAME));
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    tracing_subscriber::registry().with(debug_log).init();

    tracing::info!("Tracing initialized");
}
