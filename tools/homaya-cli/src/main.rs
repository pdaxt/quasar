use clap::Parser;

#[derive(Parser)]
#[command(name = "quasar")]
#[command(about = "QUASAR - Quantum Unified Architecture for Simulation And Runtime")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run a quantum circuit
    Run {
        /// Path to circuit file
        file: String,
    },
    /// Show version information
    Version,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Run { file }) => {
            println!("Running circuit from: {}", file);
        }
        Some(Commands::Version) => {
            println!("QUASAR v{}", env!("CARGO_PKG_VERSION"));
        }
        None => {
            println!("QUASAR - The future of quantum infrastructure");
            println!("Run 'quasar --help' for usage");
        }
    }
}
