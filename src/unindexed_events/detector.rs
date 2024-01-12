use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::loader::ContextLoader;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{Detector, IssueSeverity};

#[derive(Default)]
pub struct UnindexedEventsDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for UnindexedEventsDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // for each event definition, check if it has any indexed parameters
        // if it does not, then add it to the list of found unindexed events
        for event_definition in loader.event_definitions.keys() {
            let mut indexed_count = 0;
            let mut non_indexed = false;

            for param in &event_definition.parameters.parameters {
                if let Some(true) = param.indexed {
                    indexed_count += 1;
                } else {
                    non_indexed = true;
                }
            }

            if non_indexed && indexed_count < 3 {
                capture!(self, loader, event_definition);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Event is missing `indexed` fields")
    }

    fn description(&self) -> String {
        String::from(
            "Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod unindexed_event_tests {

    use crate::config_tests::tests_configuration;

    use super::UnindexedEventsDetector;

    use aderyn_driver::context::loader::ContextLoader;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::Detector;

    fn test_unindexed_events_for(
        _contract_file: String,
        loader: ContextLoader,
        mut detector: impl Detector,
    ) {
        // assert that the detector finds the public function
        let found = detector.detect(&loader).unwrap();
        assert!(found);
    }

    #[test]
    fn test_unindexed_events() {
        let detector = UnindexedEventsDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.title());

        for contract_file in contracts {
            let detector = UnindexedEventsDetector::default();
            let loader = load_contract(&contract_file);
            test_unindexed_events_for(contract_file, loader, detector);
        }
    }
}
