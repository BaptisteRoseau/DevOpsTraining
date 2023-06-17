# Create An Application

Because this project relies mainly upon DevOps, the application does not matter that much.
For the moment, I will create a simple HTTPS that allows CRUD actions on a HashMap, using REST standards.

## Specifications

First I need to improve my Rust skills as I already learnt some of it between the start of this project and the lines I am writing right now.

I already know a bit of [Actix Web](https://actix.rs/) so let's try [Warp](https://docs.rs/warp/latest/warp/).

As every HTTP API having a little of professionalism, this one will require:

- A CLI interface: [clap](https://docs.rs/clap/latest/clap/)
- A configuration file: [serde](https://serde.rs/) + [clap](https://docs.rs/clap/latest/clap/)
- Logging: [log](https://docs.rs/log/latest/log/)
- Custom error management: [anyhow](https://docs.rs/anyhow/latest/anyhow/) + [thiserror](https://docs.rs/thiserror/latest/thiserror/)
- A swagger: [utoipa](https://docs.rs/utoipa/latest/utoipa/)

HTTPS will be used and th application will be secured first with a security token middleware, and _maybe_ later with a login system.

The configuration parameters priority will be from:

1. CLI
2. Environment variables
3. Configuration
4. Defaults

Once this base is set up, it will be copied in my skeletons projects as I believe every decent program should have at least CLI, config file and logging.

Even better, I believe the logging should optionally be able to be configured to be compatible with normalized formats such as Syslog, CEF or LEEF, and use key-value pairs when necessary. Maybe that is because I have a SIEM developer background. Maybe I will create my own logging crate for this purpose, who know ?

### CI/CD

The binary should be compiled and released automatically through CI/CD pipelines. It will always have the _dev_ version until a new tag is created, which will be the version of the released application.

Alternatively, I would also like to manage to create architecture-specific packages for at least amd64 and arm64, a source code and a windows release.

If possible, I would like to find a way to reduce network load by caching downloaded crates directly through the _runner_ cache, not the pipeline.

## Set Up The Project

First, install Rust. The [official installer](https://www.rust-lang.org/tools/install) recommends using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install Rust in your home directory in `.cargo` and `.rustup`.

Then, run the following command to set up a brand-new project.

```bash
cargo new application
```

More information in the [Rust Book](https://doc.rust-lang.org/book/). I have already read half of it a strongly recommend reading it !

## Set Up The CI/CD Pipelines

<!-- TODO -->
