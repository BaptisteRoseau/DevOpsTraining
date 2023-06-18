use crate::config::Config;
use crate::logging::init_logger;

pub(crate) fn run(config: &Config) -> Result<(), anyhow::Error> {
    init_logger(config.debug);
    Ok(())
}
