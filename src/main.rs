mod converter;
mod models;
mod utils;

use clap::{Parser, Subcommand};
use models::ConversionRecord;

#[derive(Parser)]
#[command(name = "unitconv", version = "1.0", about = "CLI Konversi Satuan Suhu dan Panjang")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
    List,
    History,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert { from, to, value } => match converter::convert(from, to, *value) {
            Ok(result) => {
                println!("{} {} = {} {}", value, from, result, to);
                let record = ConversionRecord {
                    from: from.clone(),
                    to: to.clone(),
                    value: *value,
                    result,
                };
                utils::save_history(&record);
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::List => {
            println!("Satuan yang didukung:");
            println!("1. [suhu] celsius");
            println!("2. [suhu] fahrenheit");
            println!("3. [suhu] kelvin");
            println!("4. [panjang] cm");
            println!("5. [panjang] inch");
            println!("6. [panjang] km");
            println!("7. [panjang] miles");
        }
        Commands::History => {
            let records = utils::load_history();
            if records.is_empty() {
                println!("Belum ada riwayat konversi.");
            } else {
                println!("Riwayat Konversi:");
                for (i, r) in records.iter().enumerate() {
                    println!("{}. {} {} = {} {}", i + 1, r.value, r.from, r.result, r.to);
                }
            }
        }
    }
}
