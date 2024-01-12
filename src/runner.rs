use aderyn_driver::detector::Detector;
use aderyn_driver::driver::{drive_with, Args};

// Core detectors
use aderyn_driver::detection_modules::high::delegate_call_in_loop::DelegateCallInLoopDetector;
use aderyn_driver::detection_modules::low::push_0_opcode::PushZeroOpcodeDetector;

// Custom detectors
use crate::unspecific_solidity_pragma::detector::UnspecificSolidityPragmaDetector;

// `cargo run` will run this function
pub fn run() {
    let subscriptions: Vec<Box<dyn Detector>> = vec![
        // List of detectors to run in producing report
        Box::<PushZeroOpcodeDetector>::default(),
        Box::<DelegateCallInLoopDetector>::default(),
        Box::<UnspecificSolidityPragmaDetector>::default(),
    ];

    drive_with(
        Args {
            root: "PATH-TO-PROJECT-FOLDER".to_string(),
            output: "REPORT.md".to_string(),
            exclude: None,
            scope: None,
            no_snippets: false,
        },
        subscriptions,
    )
}
