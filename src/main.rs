//todo: tracing/logging
use cli_options::parse_cli_options;
use config::Config;
use device_info::load_db;
use device_pool::spawn_device_pool_thread;
use interfaces::new_global_state;

mod cli_options;
mod config;
mod device_handler;
mod device_info;
mod device_pool;
mod interfaces;
mod utils;

const VID: u16 = 0x256c;
const PID: u16 = 0x006d;

fn main() {
    pretty_env_logger::init();

    let cli_options = parse_cli_options();
    let config = Config::load_config(&cli_options.get_config());
    let device_db = load_db(&cli_options.get_device_db());
    let global_state = new_global_state(config, device_db);

    spawn_device_pool_thread(global_state);
    //use rusb::hotplug after initial thread spawning
}
