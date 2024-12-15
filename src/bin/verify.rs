extern crate phase2;
extern crate zeos_caterpillar;
extern crate pairing;
extern crate blake2_rfc;

use std::fs::File;
use std::io::BufReader;
use blake2_rfc::blake2b::Blake2b;
use zeos_caterpillar::constants::MERKLE_TREE_DEPTH;

fn main()
{
    let params = File::open("params").unwrap();
    let mut params = BufReader::with_capacity(1024 * 1024, params);

    let mint = phase2::MPCParameters::read(&mut params, true).expect("couldn't deserialize Mint params");
    let spend_output = phase2::MPCParameters::read(&mut params, true).expect("couldn't deserialize SpendOutput params");
    let spend = phase2::MPCParameters::read(&mut params, true).expect("couldn't deserialize Spend params");
    let output = phase2::MPCParameters::read(&mut params, true).expect("couldn't deserialize Output params");

    let mint_contributions = mint.verify(zeos_caterpillar::circuit::mint::Mint {
        account: None,
        auth_hash: None,
        value: None,
        symbol: None,
        contract: None,
        address: None,
        rcm: None,
        proof_generation_key: None
    }).expect("Mint parameters are invalid");

    let spend_output_contributions = spend_output.verify(zeos_caterpillar::circuit::spend_output::SpendOutput {
        note_a: None,
        proof_generation_key: None,
        auth_path: vec![None; MERKLE_TREE_DEPTH],
        rcv: None,
        rcv_mul: None,
        rscm: None,
        note_b: None,
        value_c: None,
        unshielded_outputs_hash: None
    }).expect("SpendOutput parameters are invalid");

    let spend_contributions = spend.verify(zeos_caterpillar::circuit::spend::Spend {
        note_a: None,
        proof_generation_key: None,
        auth_path: vec![None; MERKLE_TREE_DEPTH],
        rcv: None,
        rscm: None,
    }).expect("Spend parameters are invalid");

    let output_contributions = output.verify(zeos_caterpillar::circuit::output::Output {
        rcv: None,
        rscm: None,
        note_b: None,
    }).expect("Output parameters are invalid");

    for (((a, b), c), d) in mint_contributions.into_iter()
        .zip(spend_output_contributions.into_iter())
        .zip(spend_contributions.into_iter())
        .zip(output_contributions)
    {
        let mut h = Blake2b::new(64);
        h.update(&a);
        h.update(&b);
        h.update(&c);
        h.update(&d);
        let h = h.finalize();

        println!("{}", into_hex(h.as_ref()));
    }
}

fn into_hex(h: &[u8]) -> String
{
    let mut f = String::new();
    for byte in &h[..] {
        f += &format!("{:02x}", byte);
    }
    f
}
