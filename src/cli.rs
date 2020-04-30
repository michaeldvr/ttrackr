// cli args parser
use chrono::NaiveDate;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
// #[structopt(setting = AppSettings::InferSubcommands)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(StructOpt, Debug)]
enum Sub {
    #[structopt(name = "create")]
    Create(CreateOpts),
    #[structopt(name = "remove")]
    Remove(RemoveOpts),
    #[structopt(name = "start")]
    Start(StartOpts),
    #[structopt(name = "stop")]
    Stop(StopOpts),
}

#[derive(StructOpt, Debug)]
#[structopt(visible_alias = "new")]
struct CreateOpts {
    #[structopt(help = "New project name")]
    name: String,
    #[structopt(short = "t", help = "Target duration in minutes")]
    duration: Option<u32>,
    #[structopt(short = "d", help = "Due date")]
    duedate: Option<NaiveDate>,
    #[structopt(short = "f", long = "flat", help = "Flat project type")]
    task: Option<Option<String>>,
}

#[derive(StructOpt, Debug)]
#[structopt(visible_alias = "del")]
struct RemoveOpts {
    #[structopt(help = "Project name")]
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

pub fn parse_cli() {
    let args = Cli::from_args();
    println!("Hello, world!");
    println!("{:?}", &args);
    match &args.cmd {
        Sub::Create(cfg) => {
            println!("{:?}", &cfg);
            println!("{}", &cfg.name);
        }
        _ => (),
    };
}
