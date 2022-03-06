use crate::application::Application;
use adw::prelude::*;

mod application;
mod window;
mod config;

fn main() {
    let app = Application::new(config::APP_ID);

    app.run();
}
