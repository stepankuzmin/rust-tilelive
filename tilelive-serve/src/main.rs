use std::io;

use serde::Deserialize;

use tilelive::Tilelive;

mod sources;

const USAGE: &'static str = "
tilelive-serve

Usage:
  tilelive-serve <uri>...
  tilelive-serve (-h | --help)
  tilelive-serve --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_uri: Vec<String>,
}

fn main() -> io::Result<()> {
    use actix_web::{http, middleware, App, HttpServer};
    use docopt::Docopt;
    use listenfd::ListenFd;

    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let uri = args.arg_uri.first().unwrap();

    let mut tl = Tilelive::new();
    tl.load(uri)?;

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let cors_middleware = middleware::DefaultHeaders::new()
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*");

        App::new()
            .data(tl.clone())
            .wrap(cors_middleware)
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(sources::config)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run()
}
