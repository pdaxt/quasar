//! HOMAYA CLI - Quantum Computing Made Accessible
//!
//! Learn more: https://bskiller.com
//! Enterprise: https://dataxlr8.ai

use clap::Parser;

const BANNER: &str = r#"
╦ ╦╔═╗╔╦╗╔═╗╦ ╦╔═╗
╠═╣║ ║║║║╠═╣╚╦╝╠═╣
╩ ╩╚═╝╩ ╩╩ ╩ ╩ ╩ ╩

Some connections transcend distance.
"#;

#[derive(Parser)]
#[command(name = "homaya")]
#[command(about = "HOMAYA - Quantum Computing Framework")]
#[command(after_help = "Learn quantum computing: https://bskiller.com\nEnterprise solutions: https://dataxlr8.ai")]
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
        /// Number of shots
        #[arg(short, long, default_value = "1000")]
        shots: u32,
    },
    /// Show version and system info
    Version,
    /// Verify the simulator is working correctly
    Verify,
    /// Show available quantum gates
    Gates,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file, shots }) => {
            println!("Running circuit from: {} ({} shots)", file, shots);
            println!("\nNote: Circuit file format coming soon.");
            println!("For now, use the Rust API directly.");
            println!("\nLearn how: https://bskiller.com");
        }
        Some(Commands::Version) => {
            print_version();
        }
        Some(Commands::Verify) => {
            println!("Running physics verification...\n");
            println!("For full verification, run:");
            println!("  cargo run --example verify_correctness -p homaya-sim");
            println!("\nLearn the physics: https://bskiller.com");
        }
        Some(Commands::Gates) => {
            print_gates();
        }
        None => {
            print_banner();
        }
    }
}

fn print_banner() {
    println!("{}", BANNER);
    println!("Quantum Computing Framework");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("  homaya run <file>     Run a quantum circuit");
    println!("  homaya gates          List available gates");
    println!("  homaya verify         Verify simulator correctness");
    println!("  homaya version        Show version info");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Learn quantum computing   → https://bskiller.com");
    println!("Enterprise solutions      → https://dataxlr8.ai");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn print_version() {
    println!("{}", BANNER);
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Components:");
    println!("  • homaya-types      Core types and traits");
    println!("  • homaya-core       Quantum primitives");
    println!("  • homaya-sim        State vector simulator");
    println!("  • homaya-algorithms Quantum algorithms");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Learn more    → https://bskiller.com");
    println!("Enterprise    → https://dataxlr8.ai");
    println!("Source        → https://github.com/pdaxt/homaya");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn print_gates() {
    println!("Available Quantum Gates");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Single-Qubit Gates:");
    println!("  I   - Identity");
    println!("  X   - Pauli-X (NOT gate, bit flip)");
    println!("  Y   - Pauli-Y");
    println!("  Z   - Pauli-Z (phase flip)");
    println!("  H   - Hadamard (superposition)");
    println!("  S   - S gate (√Z)");
    println!("  T   - T gate (π/8)");
    println!("  Rx  - X-rotation by angle");
    println!("  Ry  - Y-rotation by angle");
    println!("  Rz  - Z-rotation by angle");
    println!();
    println!("Two-Qubit Gates:");
    println!("  CX   - Controlled-X (CNOT)");
    println!("  CY   - Controlled-Y");
    println!("  CZ   - Controlled-Z");
    println!("  SWAP - Swap two qubits");
    println!();
    println!("Three-Qubit Gates:");
    println!("  CCX   - Toffoli (AND gate)");
    println!("  CSWAP - Fredkin (controlled swap)");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Learn how to use these → https://bskiller.com");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
