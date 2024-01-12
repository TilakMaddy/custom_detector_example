use crate::unindexed_events::detector::UnindexedEventsDetector;
// ADERYN-PILOT: 0X01 (Please feel free to fix above imports if they mess up)

/**
 * IMPORTANT
 *  - Do not EVER remove any comments that start with ADERYN-PILOT: 0x
 *  - Do not add any comments of your own, change function definitions, etc
 *  - However, YOU ARE ALLOWED to modify the custom_detectors array so long as you maintain the original structure.
 */
// ADERYN-PILOT: 0x02 BASIC IMPORTS
use aderyn_driver::detector::Detector;

// ADERYN-PILOT: 0x03 fn custom_detectors
pub fn custom_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        // ADERYN-PILOT: 0x04 CUSTOM DETECTORS - Do not remove this comment even if the array is empty
        Box::<UnindexedEventsDetector>::default(),
    ]
}
