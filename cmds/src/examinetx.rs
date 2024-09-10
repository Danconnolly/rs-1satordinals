use bitcoinsv::bitcoin::{FromHex, Tx};
use clap::Parser;
use simple_logger::SimpleLogger;
use one_sat_ordinals::OrdinalInscription;

/// Extract 1SatOrdinals data from a transaction.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The transaction in hex format.
    #[clap(index=1)]
    tx: String,
    #[clap(long, short, action)]
    trace: bool,
}


fn main() {
    let args: Args = Args::parse();
    if args.trace {
        simple_logger::init_with_level(log::Level::Warn).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Warn).unwrap();
    }
    let r = Tx::from_hex(args.tx);
    match r {
        Err(err) => { println!("Error parsing tx, {}", err); }
        Ok(tx) => {
            println!("tx hash: {}", tx.hash());
            match OrdinalInscription::scan_tx(&tx) {
                Err(err) => { println!("Error scanning for inscriptions, {}", err); }
                Ok(v) => {
                    println!("found {} inscriptions", v.len());
                    for t in v {
                        println!("{:?}", t);
                    }
                }
            }

        }
    }
}
