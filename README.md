# zeos-caterpillar-mpc

This code can be used to participate in and verify the `zeos-caterpillar` multi-party computation (MPC). The result of this MPC is the zk-parameter files of the arithmetic circuits of the ZEOS Caterpillar Shielded Protocol. The code can be easily adapted to any other zk-project which is based on [bellman](https://github.com/zkcrypto/bellman) version [![Crates.io](https://img.shields.io/crates/v/bellman.svg)](https://crates.io/crates/bellman). 

A big shoutout and thanks to the amazing Zcash engineers of the Electric Coin company! Thanks to their awesome open-source codebase projects like ZEOS become possible.

## The MPC Process

Follow the steps below in order to conduct the MPC (aka 'trusted ceremony'). First you will need to clone this repository (all participants have to). Open a terminal and run `git clone https://github.com/mschoenebeck/zeos-caterpillar-mpc`. Then `cd zeos-caterpillar-mpc` to change into the directory.

1) In order to start a new MPC execute: `cargo run --release --bin new --features="verification"` This will run for a little while. If you are the one executing this first step to initiate the MPC you will have to download the [Powers Of Tau](https://zfnd.org/announcing-the-worlds-largest-multi-party-computation-ceremony/) parameter files and place them into the directory of `zeos-caterpillar-mpc`. You can download the files via torrent at this magnet link: `magnet:?xt=urn:btih:c3f316242ff3f4c2ec5f8cbfc27ae2ca2b599146&dn=powersoftau&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce` This will create the initial `params` file. The resulting `params` file is sent to the first participant of the MPC.

2) For each MPC participant, repeat the following steps:
  - (optional) Verify received `params` file: `cargo run --release --bin verify --features="verification"` This will run for a little while. Check the list of output hashes and make sure it matches with the list of hashes provided by the previous participants. (If you are the first participant you can skip this step.)
  - Contribute yourself: `cargo run --release --bin compute` This will run for a little while. This is the step in which you contribute entropy to the `params` file. This step will create a `new_params` file and spit out a hash on the console once the process is completed. Make sure to store your hash value.
  - (optional) Verify your own contribution: `cargo run --release --bin verify_transform --features="verification"` This will run for a little while. Once completed it should print the same hash to the console which you already stored after the previous `compute` step. If they don't match something went wrong.
  - Delete the old `params` file. Then rename the `new_params` file to `params` and share it (together with your contribution hash) with the community.

3) Once all participants have contributed the `params` file is finalized by calling: `cargo run --release --bin beacon --features="beacon"` This step is very similar to the `compute` step except that the entropy which is added here originates from a future but deterministic event. A great source for the `beacon` entropy is a Bitcoin block hash of a block which is going to be mined in the future (and thus fully deterministic but unknown to all participants at the time of the MPC). Anyone can execute this final step to finalize the `params` file.

4) Finally, the MPC is completed and the `params` need to be split into the separate circuit parameter files. In order to do this, call: `cargo run --release --bin split_params` which will take the final `params` file and split it into `x` files where `x` is the number of arithmetic circuits (in case of `zeos-caterpillar` this number is 4).

Eventually four files are being generated. Once for each arithmetic circuit of the protocol:
- `mint.params`
- `spend_output.params`
- `spend.params`
- `output.params`

## How to participate?

Join our [telegram group](https://t.me/ZeosOfficial) in order to participate. You'll need the latest (stable) [Rust compiler](https://www.rust-lang.org/) to participate using this code.

If you are on Windows, please download and execute: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

If you are on Linux or MacOS, open a terminal and execute: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Download/Clone this repository: `git clone https://github.com/mschoenebeck/zeos-caterpillar-mpc`

Open a terminal and change the directory to the just downloaded repository: `cd zeos-caterpillar-mpc`

When it's your turn, you'll receive a `params` file from whoever is the participant before you. Place this file in the current directory and run:

```
cargo run --release --bin compute
```

This will compute for a little while, and then spit out a `new_params` file. Delte the old `params` file and rename the `new_params` file to `params`. That's what you'll share back with the community. The tool also prints a hash. This hash is what you and others will use later to verify that your individual contribution actually ended up in the final parameters. So please share it together with your `params` file back to the community! For more details see point 2 above.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
