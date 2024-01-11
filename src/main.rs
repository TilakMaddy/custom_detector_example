// This import is required at minimum
use aderyn_driver::driver::{self, Args};

// These import are for subscribing to desired detectors
use aderyn_driver::detector::Detector;
use aderyn_driver::detection_modules::high::arbitrary_transfer_from::ArbitraryTransferFromDetector;
use aderyn_driver::detection_modules::low::push_0_opcode::PushZeroOpcodeDetector;

// This import is for the custom detector `unindexed_events.rs`
use custom_detector_example::unindexed_events::UnindexedEventsDetector;

use std::path::PathBuf;

fn main() {

    let root_path = PathBuf::from("/Users/tilakmadichetti/Documents/EthereumProjects/simple_storage");

    //////////////////////  DEFAULT (EVERYTHING) //////////////////////////////////

    driver::drive(Args{
        root: root_path.to_str().unwrap().to_string(),
        output: "default_analysis_report.md".to_string(),
        exclude: None,
        no_snippets: false,
        scope: None,
    });    

    ////////////////////// SUBSCRIBE TO INTERESTED ONES ///////////////////////////
    
    let subscribe_to: Vec<Box<dyn Detector>> = vec![
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<PushZeroOpcodeDetector>::default(),
    ];
    
    driver::drive_with( // notice this is `drive_with` unlike like above
        Args{
            root: root_path.to_str().unwrap().to_string(),
            output: "subscription_analysis_report.md".to_string(),
            exclude: None,
            no_snippets: false,
            scope: None,
        },
        subscribe_to  // inject subscriptions here
    );    

    //////////////////// HYBRID (CUSTOM ONE + aderyn) /////////////////////////////
    
    // There is a file called `unindexed_events` - let's pretend it is the custom written 
    // detector. Now we want to use that along with   `ArbitraryTransferFromDetector`

    let subscribe_to_hybrid: Vec<Box<dyn Detector>> = vec![
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<UnindexedEventsDetector>::default(),
    ];

    driver::drive_with( // notice this is `drive_with` unlike like the first ex.
        Args{
            root: root_path.to_str().unwrap().to_string(),
            output: "custom_subscription_analysis_report.md".to_string(),
            exclude: None,
            no_snippets: false,
            scope: None,
        },
        subscribe_to_hybrid  // inject subscriptions here
    );   

}