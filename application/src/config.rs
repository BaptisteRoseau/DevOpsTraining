use clap::Parser;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PORT: u16 = 6969;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Config {
    /// Path to the configuration file
    #[arg(short, long, env)]
    pub config: Option<PathBuf>,

    /// Enable debug logging
    #[arg(long, env, default_value_t = false)]
    pub debug: bool,

    /// The IP where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_IP)]
    pub ip: IpAddr,

    /// The port where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub port: u16,
}
