use std::collections::BTreeMap;
use std::process::Command;
use chrono::Utc;
use tracing_subscriber::util::SubscriberInitExt;
use zellij_tile::prelude::*;

static TIMEOUT_INTERVAL: f64 = 2.0;

#[derive(Default)]
struct State {
    output: String,
    last_update: f64,
    userspace_configuration: BTreeMap<String, String>,
    has_permissions: bool,
}

register_plugin!(State);

// zellij plugin development materials
// docs : https://zellij.dev/docs/overview
// guide: https://web.archive.org/web/20240903124717/https://blog.nerd.rocks/posts/common-snippets-for-zellij-development/

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        init_tracing();
        self.userspace_configuration = configuration;
        self.has_permissions = false;
        // initialize last_update to 0 to force an update on the first render
        self.last_update = 0.0;
        request_permission(&[
            PermissionType::RunCommands,
        ]);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::Timer,
            EventType::PermissionRequestResult,
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
                // so it is not selectable anyways.
                set_selectable(false);
                set_timeout(TIMEOUT_INTERVAL);
            }
            Event::Timer(_time) => {
                tracing::debug!("Event Timer received, time: {}", _time);
                // now
                let now = Utc::now().timestamp() as f64;
                // Update timezone every minute
                if now - self.last_update >= 4.0 {
                    tracing::debug!("Time to update");
                    self.refresh_last_update();
                    self.output = get_datetime_to_show();
                    should_render = true;
                } else {
                  tracing::debug!("Too soon, won't update!");
                }
                set_timeout(TIMEOUT_INTERVAL);
            }
            _ => {}
        }
        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("This is a drill!!!");
        println!("last_update: {}", self.last_update.to_string());
        println!("output: {}", self.output);
    }
}

impl State {
    fn refresh_last_update(&mut self) {
        self.last_update = get_current_time();
    }
}

fn get_datetime_to_show() -> String {
    let output = Command::new("date")
        .arg("+%Y/%m/%d %H:%M:%S")
        .output()
        .expect("Failed to execute date command");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_current_time() -> f64 {
    // when compiling to WebAssembly, code runs in a sandboxed environment.
    // The behavior of SystemTime will depend on the implementation of the
    // WebAssembly runtime and the host environment.
    // std::time::SystemTime::now()
    //     .duration_since(std::time::UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs_f64()
    Utc::now().timestamp() as f64
}

fn init_tracing() {
    use std::fs::File;
    use std::sync::Arc;
    use tracing_subscriber::layer::SubscriberExt;

    let file = File::create(".zellij_plugin.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    tracing_subscriber::registry().with(debug_log).init();

    tracing::info!("tracing initialized");
}
