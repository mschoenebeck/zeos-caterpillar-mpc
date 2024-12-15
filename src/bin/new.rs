extern crate phase2;
extern crate zeos_caterpillar;
extern crate pairing;

use std::fs::File;
use std::io::BufWriter;
use zeos_caterpillar::constants::MERKLE_TREE_DEPTH;

fn main()
{
    let params = File::create("params").unwrap();
    let mut params = BufWriter::with_capacity(1024 * 1024, params);

    // Mint circuit
    phase2::MPCParameters::new(zeos_caterpillar::circuit::mint::Mint {
        account: None,
        auth_hash: None,
        value: None,
        symbol: None,
        contract: None,
        address: None,
        rcm: None,
        proof_generation_key: None
    }).unwrap().write(&mut params).unwrap();

    // SpendOutput circuit
    phase2::MPCParameters::new(zeos_caterpillar::circuit::spend_output::SpendOutput {
        note_a: None,
        proof_generation_key: None,
        auth_path: vec![None; MERKLE_TREE_DEPTH],
        rcv: None,
        rcv_mul: None,
        rscm: None,
        note_b: None,
        value_c: None,
        unshielded_outputs_hash: None
    }).unwrap().write(&mut params).unwrap();

    // Spend circuit
    phase2::MPCParameters::new(zeos_caterpillar::circuit::spend::Spend {
        note_a: None,
        proof_generation_key: None,
        auth_path: vec![None; MERKLE_TREE_DEPTH],
        rcv: None,
        rscm: None,
    }).unwrap().write(&mut params).unwrap();

    // Output circuit
    phase2::MPCParameters::new(zeos_caterpillar::circuit::output::Output {
        rcv: None,
        rscm: None,
        note_b: None,
    }).unwrap().write(&mut params).unwrap();
}
