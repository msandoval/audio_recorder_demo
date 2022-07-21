mod audio_clip;
use clap::{AppSettings, Parser, Subcommand};
use color_eyre::eyre::{ErrReport, Result};
use audio_clip::AudioClip;

#[derive(Parser, Debug)]
#[clap(name="audio_recorder")]
#[clap(about = "A voice application")]

struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Record an audio clip using the default input device
    Record {
        /// The name of the clip to record. If not specified, the current date will be used
        name: Option<String>,
    },
    ///List all clips
    List {},
    /// Play clip with the given name
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Play {
        /// Name of the clip to play
        name: String
    },
    ///Delete the clip with the given name
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Delete {
        name: String
    }
}
fn main() -> Result<()> {
    color_eyre::install();
    let args = Cli::parse();

    match args.command {
        Commands::Record { name } => {
            let name = name.unwrap_or_else(|| "<untitled>".to_string());
            let clip = AudioClip::record()?;
            clip.play()?;
            todo!();
        }
        Commands::List {} => {
            todo!();
        }
        Commands::Play { name } => {
            eprintln!("Play: {}", name);
            todo!();
        }
        Commands::Delete { name} => {
            eprintln!("Play: {}", name);
            todo!();
        }
    }
}
