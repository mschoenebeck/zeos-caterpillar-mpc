extern crate phase2;
extern crate pairing;
extern crate rand;
extern crate rand_core;
extern crate blake2_rfc;

use std::fs::File;
use std::io::{BufWriter, BufReader};
use blake2_rfc::blake2b::Blake2b;
use rand_core::OsRng;

fn main()
{
    let current_params = File::open("params").expect("couldn't open `./params`");
    let mut current_params = BufReader::with_capacity(1024*1024, current_params);

    let new_params = File::create("new_params").expect("couldn't create `./new_params`");
    let mut new_params = BufWriter::with_capacity(1024*1024, new_params);

    let mut mint = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize Mint params");
    let mut spend_output = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize SpendOutput params");
    let mut spend = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize Spend params");
    let mut output = phase2::MPCParameters::read(&mut current_params, false).expect("couldn't deserialize Output params");

    let mut rng = OsRng.clone();

    let h1 = mint.contribute(&mut rng);
    let h2 = spend_output.contribute(&mut rng);
    let h3 = spend.contribute(&mut rng);
    let h4 = output.contribute(&mut rng);

    mint.write(&mut new_params).expect("couldn't write new Mint params");
    spend_output.write(&mut new_params).expect("couldn't write new SpendOutput params");
    spend.write(&mut new_params).expect("couldn't write new Spend params");
    output.write(&mut new_params).expect("couldn't write new Output params");

    let mut h = Blake2b::new(64);
    h.update(&h1);
    h.update(&h2);
    h.update(&h3);
    h.update(&h4);
    let h = h.finalize();

    print!("Done!\n\n\
              Your contribution has been written to `./new_params`\n\n\
              The contribution you made is bound to the following hash:\n");

    println!("{}", into_hex(h.as_ref()));
}

fn into_hex(h: &[u8]) -> String
{
    let mut f = String::new();
    for byte in &h[..] {
        f += &format!("{:02x}", byte);
    }
    f
}
