use std::path::PathBuf;

use aderyn_core::detect::detector::Detector;
use aderyn_core::run_with_detectors;
use aderyn::process_foundry;
use aderyn::tokei::{self, Config, LanguageType};
use custom_detector_example::unsafe_detector::UnsafeERC20FunctionsDetector;

fn main() {

    let root_path = PathBuf::from("/Users/tilakmadichetti/Documents/EthereumProjects/simple_storage");

    let (src_path, mut context_loader) = process_foundry::with_project_root_at(&root_path);

    let my_detectors: Vec<Box<dyn Detector>> = vec![
        Box::<UnsafeERC20FunctionsDetector>::default()
    ];

    {
        // Using the source path, get the sloc from tokei
        let mut languages = tokei::Languages::new();
        let tokei_config = Config::default();
        languages.get_statistics(&[src_path], &[], &tokei_config);
        context_loader.set_sloc_stats(languages[&LanguageType::Solidity].clone());
    }

    run_with_detectors(
        context_loader,
        "analysis_output.md".to_owned(), 
        my_detectors
    ).unwrap();

}
