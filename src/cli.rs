// cli args parser
use crate::config;
use crate::utils::BoxError;
use chrono::NaiveDate;
use log::debug;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::db::ops;

#[derive(StructOpt, Debug)]
// #[structopt(setting = AppSettings::InferSubcommands)]
struct Cli {
    #[structopt(parse(from_os_str), help = "config file", long)]
    config: Option<PathBuf>,
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(StructOpt, Debug)]
enum Sub {
    #[structopt(name = "create", visible_alias = "new")]
    Create(CreateOpts),
    #[structopt(name = "edit")]
    Edit(EditOpts),
    #[structopt(name = "delete", visible_alias = "del")]
    Delete(DeleteOpts),
    #[structopt(name = "start")]
    Start(StartOpts),
    #[structopt(name = "stop")]
    Stop(StopOpts),
    #[structopt(name = "stopall")]
    StopAll(StopAllOpts),
    #[structopt(name = "status", visible_alias = "stat")]
    Status(StatusOpts),
    #[structopt(name = "list")]
    List(ListOpts),
}

#[derive(StructOpt, Debug)]
struct CreateOpts {
    #[structopt(help = "New task name")]
    name: String,
    #[structopt(short = "t", help = "Target duration in minutes")]
    duration: Option<i32>,
    #[structopt(short = "d", help = "Due date")]
    duedate: Option<NaiveDate>,
    #[structopt(short = "n", long = "note", help = "Description")]
    task: Option<String>,
}

#[derive(StructOpt, Debug)]
struct EditOpts {
    #[structopt(help = "Task name")]
    name: String,
    #[structopt(short = "t", help = "Target duration in minutes")]
    duration: Option<i32>,
    #[structopt(short = "d", help = "Due date")]
    duedate: Option<NaiveDate>,
    #[structopt(short = "n", long = "note", help = "Description")]
    task: Option<String>,
}

#[derive(StructOpt, Debug)]
struct DeleteOpts {
    #[structopt(help = "Task name")]
    name: String,
    #[structopt(long, help = "Skip confirmation")]
    noconfirm: bool,
}

#[derive(StructOpt, Debug)]
struct StartOpts {
    #[structopt(help = "Task name(s)")]
    name: Vec<String>,
}

#[derive(StructOpt, Debug)]
struct StopOpts {
    #[structopt(help = "Task name(s)")]
    name: Vec<String>,
}

#[derive(StructOpt, Debug)]
struct StopAllOpts {}

#[derive(StructOpt, Debug)]
struct StatusOpts {
    #[structopt(short = "f", long = "filter", name = "task name")]
    filter: Option<String>,
}

#[derive(StructOpt, Debug)]
struct ListOpts {}

pub fn parse_cli() -> Result<(), BoxError> {
    let args = Cli::from_args();
    debug!("Hello, world!");
    debug!("{:?}", &args);

    let conf: Option<&PathBuf> = match &args.config {
        Some(val) => Some(&val),
        None => None,
    };

    let cfgpath = match config::create_config(conf) {
        Ok((created, path)) => {
            if created {
                debug!("Created config file at {:?}", path);
            }
            path
        }
        Err(err) => {
            return Err(err);
        }
    };

    let config = config::Config::load(Some(&cfgpath))?;

    match &args.cmd {
        Sub::Create(args) => {
            let duration = match args.duration {
                Some(val) => Some(val * 60), // mins to secs
                None => None,
            };
            ops::create_task(&config, &args.name, None, duration)?
        }
        _ => (),
    };

    Ok(())
}
