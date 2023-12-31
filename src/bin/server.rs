use core::time;
use log::{log_enabled, Level};
use poncu::server::core::{PoncuTcpServer, TcpServer};
use poncu::utils::config;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!(
        "{} server v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let config = config::get_config();
    let server = PoncuTcpServer::with_config(&config);

    let flag_server_ready = Arc::new(AtomicBool::new(false));
    let flag_server_shutdown = Arc::new(AtomicBool::new(false));
    server.start(&flag_server_shutdown, &flag_server_ready);

    while !flag_server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }

    if log_enabled!(Level::Trace) {
        log::trace!("server ready.");
    }
}
