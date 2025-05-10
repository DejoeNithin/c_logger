use log::{Level, Metadata, Record};
use chrono::Local;

pub struct CLogger;

impl log::Log for CLogger {

  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Trace
  }

  fn flush(&self) {}

  fn log(&self, record: &Record) {
    // println!("Recieved record : {}", record.args());
    // println!("R level : {}", record.level());

    if !self.enabled(record.metadata()) {
      return
    }

    let level = match record.level() {
      Level::Error => "\x1B[1;31mERROR\x1B[0m",
      Level::Warn => "\x1B[1;33mWARN\x1B[0m",
      Level::Info => "\x1B[1;32mINFO\x1B[0m",
      Level::Debug => "\x1B[1;36mDEBUG\x1B[0m",
      Level::Trace => "\x1B[1;35mTRACE\x1B[0m",
    };

    println!("{} [{}:{}] - {}: {}",
      Local::now().format("%Y-%m-%d %H:%M:%S"),
      record.file().unwrap_or("unknown"),
      record.line().unwrap_or(0),
      level,
      record.args()
    );
  }

}

pub fn init_logger() -> Result<(), log::SetLoggerError> {
  log::set_boxed_logger(Box::new(CLogger))
    .map(|()| log::set_max_level(log::LevelFilter::Trace))
}