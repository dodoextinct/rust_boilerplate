use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs{
    /// word you want to search
    pub query: String,

    /// file where you want to search
    pub filename: String,
}