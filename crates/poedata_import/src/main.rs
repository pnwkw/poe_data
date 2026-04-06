use clap::Parser;

mod poedat;
use crate::poedat::PoEDatViewer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print game version
    #[arg(short, long)]
    game_version: bool,

    /// Update game data
    #[arg(short, long)]
    update: bool,
}

const DATA_DIR: &str = "./data";
const OUTPUT_DIR: &str = "./crates/poedata_db/src";

fn main() {
    let args = Args::parse();

    let viewer = PoEDatViewer::new(DATA_DIR);

    if args.game_version {
        println!("{}", viewer.patch);
    } else if args.update {
        viewer.fetch_game_data();
        viewer.write_cbor(OUTPUT_DIR);
    }
}
