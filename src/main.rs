//todo: tracing/logging
use cli_options::parse_cli_options;
use config::Config;
use device_info::load_db;
use device_pool::spawn_device_pool_thread;
use interfaces::new_global_state;
use udev::install_udev_rules;
use utils::print_huion_device_input;

mod cli_options;
mod config;
mod device_handler;
mod device_info;
mod device_pool;
mod event_handler;
mod interfaces;
mod platform;
mod tablet_device;
// todo: linux only
mod udev;
mod utils;

fn main() {
    pretty_env_logger::init();

    let cli_options = parse_cli_options();
    let config = Config::load_config(&cli_options.get_config());
    let device_db = load_db(&cli_options.get_device_db());
    let global_state = new_global_state(config, device_db);

    match cli_options.command {
        cli_options::Command::Run | cli_options::Command::Deamon => {
            spawn_device_pool_thread(global_state);
        }
        cli_options::Command::Install => {
            install_udev_rules();
        }
    }
}
