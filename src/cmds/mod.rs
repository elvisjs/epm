use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Create a new substrate package in an existing directory
    #[structopt(alias = "init")]
    Init,
    /// Create a new substrate package
    #[structopt(alias = "new")]
    New {
        /// Package path
        #[structopt(name = "PATH")]
        path: PathBuf,
    },
}

/// Exec commands
pub fn exec() {
    let opt = Opt::from_args();
    match opt {
        Opt::Init => {
            println!("epm init command execed");
        }
        Opt::New { path } => {
            println!("epm new command execed, the dir is {:?}", path);
        }
    }
}
