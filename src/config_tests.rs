use crate::{
    bot_utils::{TestsConfig, TestsTarget},
    unindexed_events::detector::UnindexedEventsDetector,
    unspecific_solidity_pragma::detector::UnspecificSolidityPragmaDetector,
};

pub fn tests_configuration() -> TestsConfig {
    vec![
        // Define your targets here
        TestsTarget::new("../TEST-CONTRACTS/out/xyz.sol/xyz.json")
            .with(Box::<UnindexedEventsDetector>::default())
            .with(Box::<UnspecificSolidityPragmaDetector>::default()),
    ]
    .into()
}
