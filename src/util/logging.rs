/*
* Thank you to Cassy343 for writing a lot of this logging code for our project Quartz that I stole for this
*/

use std::io::Write;
use std::error::Error;
use std::sync::Arc;
use std::io::Stdout;
use std::fmt;
use chrono::Local;
use log::{LevelFilter, Level};
use log4rs::append::file::FileAppender;
use log4rs::append::Append;
use log4rs::encode::pattern::{PatternEncoder};
use log4rs::config::{Appender, Config, Root};
use log4rs::filter::{Filter, Response};
use log::*;

#[cfg(unix)]
use termion::color;

#[cfg(debug_assertions)]
const LEVEL_FILTER: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LEVEL_FILTER: LevelFilter = LevelFilter::Info;

pub fn init_logging() {
	let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new("[{d(%H:%M:%S)} {l}]: {m}\n"))).build("log/latest.log").unwrap();
	let console = CustomConsoleAppender { console_interface: Arc::new(std::io::stdout())};

	let config = Config::builder()
			.appender(Appender::builder().filter(Box::new(CrateFilter::new("modolumia"))).build("console", Box::new(console)))
			.appender(Appender::builder().filter(Box::new(CrateFilter::new("modolumia"))).build("logfile", Box::new(logfile)))
			.build(
				Root::builder()
					.appender("console")
					.appender("logfile")
					.build(LEVEL_FILTER)
			).unwrap();

	log4rs::init_config(config).unwrap();
	info!("Initialized loggging");
	debug!("Currently running in a debug environment");
}

struct CustomConsoleAppender {
	console_interface: Arc<Stdout>
}

impl Append for CustomConsoleAppender {
	#[cfg(unix)]
	fn append(&self, record: &Record) -> Result<(), Box<dyn Error + Sync + Send>> {
		let mut writer = self.console_interface.lock();
		match record.metadata().level() {
			Level::Error => write!(writer, "{}", color::Fg(color::Red))?,
			Level::Warn => write!(writer, "{}", color::Fg(color::LightYellow))?,
			Level::Debug => write!(writer, "{}", color::Fg(color::LightCyan))?,
			_ => write!(writer, "{}", color::Fg(color::Reset))?,
		}
		writeln!(writer, "[{} {}]: {}{}", Local::now().format("%H:%M:%S"), record.metadata().level(), record.args(), color::Fg(color::Reset))?;
		Ok(())
	}

	#[cfg(not(unix))]
	fn append(&self, record: &Record) -> Result<(), Box<dyn Error + Sync + Send>> {
		let mut writer = self.console_interface.lock();
		writeln!(writer, "[{} {}]: {}", Local::now().format("%H:%M:%S"), record.metadata().level(), record.args())?;
		Ok(())
	}

	fn flush(&self) { }
}

impl fmt::Debug for CustomConsoleAppender {
	fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Ok(())
	}
}

#[cfg(debug_assertions)]
struct CrateFilter {
    filter: String
}

#[cfg(not(debug_assertions))]
struct CrateFilter;

impl CrateFilter {
    #[cfg(debug_assertions)]
    pub fn new(filter: &str) -> Self {
        CrateFilter {
            filter: filter.to_owned()
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn new(_filter: &str) -> Self {
        CrateFilter
    }
}

impl Filter for CrateFilter {
    #[cfg(debug_assertions)]
    fn filter(&self, record: &Record) -> Response {
        if record.level() != Level::Debug && record.level() != Level::Trace {
            return Response::Accept;
        }

        match record.module_path() {
            Some(path) => {
                if path.starts_with(&self.filter) {
                    Response::Accept
                } else {
                    Response::Reject
                }
            },
            None => Response::Reject
        }
    }

    #[cfg(not(debug_assertions))]
    fn filter(&self, _record: &Record) -> Response {
        Response::Neutral
    }
}

impl fmt::Debug for CrateFilter {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}