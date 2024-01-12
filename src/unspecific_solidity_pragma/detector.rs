use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::loader::ContextLoader;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{Detector, IssueSeverity};

#[derive(Default)]
pub struct UnspecificSolidityPragmaDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for UnspecificSolidityPragmaDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for pragma_directive in loader.pragma_directives.keys() {
            for literal in &pragma_directive.literals {
                if literal.contains('^') || literal.contains('>') {
                    capture!(self, loader, pragma_directive);
                    break;
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Solidity pragma should be specific, not wide")
    }

    fn description(&self) -> String {
        String::from("Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod unspecific_solidity_pragma_tests {

    use crate::config_tests::tests_configuration;

    use super::UnspecificSolidityPragmaDetector;

    use aderyn_driver::context::loader::ContextLoader;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::Detector;

    fn test_unspecific_solidity_pragma_for(
        _contract_file: String,
        loader: ContextLoader,
        mut detector: impl Detector,
    ) {
        // assert that the detector finds the public function
        let found = detector.detect(&loader).unwrap();
        assert!(found);
    }

    #[test]
    fn test_unspecific_solidity_pragma() {
        let detector = UnspecificSolidityPragmaDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.title());

        for contract_file in contracts {
            let detector = UnspecificSolidityPragmaDetector::default();
            let loader = load_contract(&contract_file);
            test_unspecific_solidity_pragma_for(contract_file, loader, detector);
        }
    }
}
