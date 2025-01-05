//! This binary just splits the parameters up into separate files.

extern crate phase2;
extern crate pairing;
extern crate rand;
extern crate blake2_rfc;
extern crate zeos_caterpillar;

use std::fs::File;
use std::io::{BufWriter, BufReader};
use zeos_caterpillar::contract::AffineVerifyingKeyBytesLE;

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
        // write out verifying key as little-endian encoded hex string for easy upload to the smart contract
        let vk_affine_bytes = AffineVerifyingKeyBytesLE::from(mint.get_params().vk.clone());
        let res = std::fs::write("mint.vk.hex", hex::encode(vk_affine_bytes.0));
        assert!(res.is_ok());
    }

    {
        let f = File::create("spend-output.params").expect("couldn't create `./spend-output.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        spend_output.write(&mut f).expect("couldn't write new SpendOutput params");
        // write out verifying key as little-endian encoded hex string for easy upload to the smart contract
        let vk_affine_bytes = AffineVerifyingKeyBytesLE::from(spend_output.get_params().vk.clone());
        let res = std::fs::write("spend-output.vk.hex", hex::encode(vk_affine_bytes.0));
        assert!(res.is_ok());
    }

    {
        let f = File::create("spend.params").expect("couldn't create `./spend.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        spend.write(&mut f).expect("couldn't write new Spend params");
        // write out verifying key as little-endian encoded hex string for easy upload to the smart contract
        let vk_affine_bytes = AffineVerifyingKeyBytesLE::from(spend.get_params().vk.clone());
        let res = std::fs::write("spend.vk.hex", hex::encode(vk_affine_bytes.0));
        assert!(res.is_ok());
    }

    {
        let f = File::create("output.params").expect("couldn't create `./output.params`");
        let mut f = BufWriter::with_capacity(1024*1024, f);
        output.write(&mut f).expect("couldn't write new Output params");
        // write out verifying key as little-endian encoded hex string for easy upload to the smart contract
        let vk_affine_bytes = AffineVerifyingKeyBytesLE::from(output.get_params().vk.clone());
        let res = std::fs::write("output.vk.hex", hex::encode(vk_affine_bytes.0));
        assert!(res.is_ok());
    }
}
