use clap::Parser;
use gdtool_rs::{DemoGenerator, TypeRegistry, process_excel};

#[derive(Parser, Debug)]
#[command(name = "gdtool-rs", about = "Godot Game Configuration Tool")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Process an Excel file and generate output
    Convert {
        /// Input Excel file path
        #[arg(short, long)]
        input: String,

        /// CSV output directory path
        #[arg(long, default_value = "data")]
        csv_out: String,

        /// Godot output directory path
        #[arg(long, default_value = "data")]
        gd_out: String,

        /// CSV extension (default: txt)
        #[arg(long, default_value = "txt")]
        csv_ext: String,
    },
    /// Generate a demo Excel file with all types
    Demo {
        /// Output file path (default: demo.xlsx)
        #[arg(short, long, default_value = "demo.xlsx")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let registry = TypeRegistry::new();

    match &cli.command {
        Commands::Convert {
            input,
            csv_out,
            gd_out,
            csv_ext,
        } => {
            println!("Processing Excel file: {}", input);
            println!("CSV output directory: {}", csv_out);
            println!("Godot output directory: {}", gd_out);

            match process_excel(input, csv_out, gd_out, csv_ext, &registry) {
                Ok(_) => {
                    println!("Success!");
                    println!("Output files:");
                    println!("  - CSV: {}", csv_out);
                    println!("  - Godot: {}", gd_out);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Demo { output } => {
            println!("Generating demo Excel file: {}", output);

            match DemoGenerator::generate_demo_excel(output, &registry) {
                Ok(_) => {
                    println!("Success! Demo Excel file created at: {}", output);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
