use std::collections::BTreeMap;
use std::process::Command;
use chrono::Utc;
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
            }
            Event::Timer(time) => {
                // Update timezone every minute
                if time - self.last_update >= 4.0 {
                    self.refresh_last_update();
                    self.output = get_datetime_to_show();
                    should_render = true;
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
        .arg("+%Y/%m/%d %H:%M")
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
