mod user;
mod auth;
use std::{fs::File, path::PathBuf, sync::Arc};

use auth::Auth;
use clap::Parser;
use libunftp::options::ActivePassiveMode;
use user::CCUser;
use unftp_sbe_rooter::RooterVfs;
use unftp_sbe_fs::{Filesystem, Meta};
use slog::{o,Drain};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Root folder of Computer Craft computers/disk
    #[arg(index=1)]
    cc_root: PathBuf,

    /// Greeting message when users connect to FTP
    #[arg(short, long, default_value = "Welcome to CCFtp")]
    greeting: String,

    /// Port number to host FTP on
    #[arg(short, long, default_value = "2121")]
    port: u16
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());

    let root = args.cc_root.canonicalize().unwrap();
    // let backend = Box::new(move || {
    //     let root2 = root.clone();
    //     unftp_sbe_rooter::RooterVfs::<Filesystem, CCUser, Meta>::new(Filesystem::new(root2))
    // });
    let backend = Box::new(move || {
        let root2 = root.clone();
        unftp_sbe_fs::Filesystem::new(root2)
    });
    let root = args.cc_root.canonicalize().unwrap();
    let server = libunftp::ServerBuilder::<Filesystem,CCUser>::with_authenticator(
            backend,
            Arc::new(Auth::new(root))
        )
        .greeting(args.greeting.leak())
        .active_passive_mode(ActivePassiveMode::ActiveAndPassive)
        .logger(log)
        .build().unwrap();

    println!("Starting server!. CC root of {:?}",args.cc_root.canonicalize().unwrap());
    let _ = server.listen(format!("127.0.0.1:{}",args.port)).await;
}

