/**
 *
 * WELCOME !
 *
 * FAQ
 *
 * > Want to create your own detectors ?
 *      - Run `aderyn_pilot g my_detector_name`
 *      - Code it out in the newly created `aderyn_pilot/detector.rs`
 *      - Write your tests
 *      - Hook up the tests with the desired solidity json out files in `config_tests.rs`
 *      - Run `cargo test`
 *
 * > Want to delete a custom detector ?
 *      - Run `aderyn_pilot d my_detector_name`
 *
 * NOTE: DO NOT MANUALLY create/delete detectors. Always use `adeyn_pilot` (it keeps track of metadata internally)
 *
 * > Want to analyze a codebase and generate your own report ?
 *      - Head over to `runner.rs`. Inside `run()`, define your subscriptions
 *      - you could include your own detectors as well as the core ones
 *      - Run `cargo run` - This will call the run() function
 *
 * ADERYN-PILOT // DO NOT TOUCH THIS FILE. - Go to `runner.rs`
 *
 * NOTE: These other flags will be used by aderyn_pilot. DO NOT MODIFY any existing
 * flags. Only if you really know what you are doing feel free to ADD new flags but by
 * any means DO NOT MODIFY / DELETE existing ones.
 *
 */
use clap::{Parser, Subcommand};
use custom_detector_example::{bot_brain, runner};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    // These are commands that will be invoked by `aderyn_pilot`.
    #[clap(subcommand, name = "pilot")]
    pilot: Option<PilotCommand>,
}

#[derive(Debug, Subcommand)]
enum PilotCommand {
    RefreshMetadata,
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    if cmd_args.pilot.is_none() {
        println!("[*] Running bot ");
        runner::run();
        return;
    }

    match cmd_args.pilot.unwrap() {
        PilotCommand::RefreshMetadata => bot_brain::refresh_metadata(),
    }
}
