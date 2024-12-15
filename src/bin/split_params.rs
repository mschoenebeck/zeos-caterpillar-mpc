//! This binary just splits the parameters up into separate files.

extern crate phase2;
extern crate pairing;
extern crate rand;
extern crate blake2_rfc;

use std::fs::File;
use std::io::{BufWriter, BufReader};

fn main() {
    let current_params = File::open("params").expect("couldn't open `./params`");
    let mut current_params = BufReader::with_capacity(1024*1024, current_params);

    let mint = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize Mint params");
    let spend_output = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize SpendOutput params");
    let spend = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize Spend params");
    let output = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize Output params");

    {
        let f = File::create("mint.params").expect("couldn't create `./mint.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        mint.write(&mut f).expect("couldn't write new Mint params");
    }

    {
        let f = File::create("spend-output.params").expect("couldn't create `./spend-output.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        spend_output.write(&mut f).expect("couldn't write new SpendOutput params");
    }

    {
        let f = File::create("spend.params").expect("couldn't create `./spend.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        spend.write(&mut f).expect("couldn't write new Spend params");
    }

    {
        let f = File::create("output.params").expect("couldn't create `./output.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        output.write(&mut f).expect("couldn't write new Output params");
    }
}
