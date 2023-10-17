use std::ffi::CString;

use log::{Metadata, LevelFilter};

static LOGGER: ObsLogger = ObsLogger;

pub fn init() {
    if let Err(e) = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace)) {
        eprintln!("failed to set logger : {}", e)
    }
}

struct ObsLogger;

impl log::Log for ObsLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!(
                "{}:{}:{} : {}",
                record.module_path().unwrap_or("unknown"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            );
            let c_message = CString::new(message).unwrap();
            let obs_level = match record.level() {
                log::Level::Trace | log::Level::Debug => crate::libobs_sys::LOG_DEBUG,
                log::Level::Info => crate::libobs_sys::LOG_INFO,
                log::Level::Warn => crate::libobs_sys::LOG_WARNING,
                log::Level::Error => crate::libobs_sys::LOG_ERROR,
            };

            unsafe {
                crate::libobs_sys::blog(obs_level as i32, c_message.as_ptr());
            }
        }
    }

    fn flush(&self) {
        // noop
    }
}
