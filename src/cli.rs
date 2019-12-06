
/// 📢 subcommands
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "average", about = "📢 subcommand to calculate average value")]
    Average(Average),
    #[structopt(name = "setup", about = "📢 subcommand to setup sensor uart setting")]
    Setup(Setting),
    #[structopt(name = "clean", about = "📢 subcommand to clean pid")]
    Clean,


}

///msgbot command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "ndir", about = "  🧰 ndir sensor interface interface usage.")]
pub struct Args {
    ///🔌 hardware connection address
    #[structopt(long = "name",  default_value = "irrigatron_bot")]
    name: String,
    ///🗁 working directory
    #[structopt(short = "d", long = "dir",  default_value = "/home/sascha/.paw/mio/ndir")]
    dir: PathBuf,
    ///📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd:Cmd,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn tocken(&self) -> String {
        if let Some(tocken)=env::var("TELEGRAM_BOT_TOKEN") {
            return token;
        }
        if self.dir.join(self.name.as_str()).is_file() {
            fs::read_to
        }
    }
    pub fn pid(&self) -> io::Result<PathBuf> {
        Ok(self.dir.join("pid"))
    }
}
