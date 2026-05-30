//! Forge Meta — Registry and discovery for the ForgeFlux ecosystem
//!
//! Knows about every forge-* crate, what it decomposes, and how to wire them together.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeCrate {
    pub name: String,
    pub version: String,
    pub description: String,
    pub domain: String,
    pub input_formats: Vec<String>,
    pub output_tile_kind: String,
    pub test_count: u32,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemReport {
    pub total_crates: usize,
    pub total_tests: u32,
    pub domains: HashMap<String, Vec<String>>,
    pub input_formats: Vec<String>,
    pub timestamp_ms: u64,
}

pub struct ForgeRegistry {
    crates: Vec<ForgeCrate>,
}

impl ForgeRegistry {
    pub fn new() -> Self {
        Self { crates: Self::builtin_crates() }
    }

    pub fn register(&mut self, crate_info: ForgeCrate) {
        self.crates.push(crate_info);
    }

    pub fn all(&self) -> &[ForgeCrate] {
        &self.crates
    }

    pub fn by_domain(&self, domain: &str) -> Vec<&ForgeCrate> {
        self.crates.iter().filter(|c| c.domain == domain).collect()
    }

    pub fn by_format(&self, format: &str) -> Vec<&ForgeCrate> {
        self.crates.iter().filter(|c| c.input_formats.iter().any(|f| f == format)).collect()
    }

    pub fn find(&self, name: &str) -> Option<&ForgeCrate> {
        self.crates.iter().find(|c| c.name == name)
    }

    pub fn ecosystem_report(&self) -> EcosystemReport {
        let mut domains: HashMap<String, Vec<String>> = HashMap::new();
        let mut formats = std::collections::HashSet::new();
        for c in &self.crates {
            domains.entry(c.domain.clone()).or_default().push(c.name.clone());
            for f in &c.input_formats {
                formats.insert(f.clone());
            }
        }
        EcosystemReport {
            total_crates: self.crates.len(),
            total_tests: self.crates.iter().map(|c| c.test_count).sum(),
            domains,
            input_formats: formats.into_iter().collect(),
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
        }
    }

    pub fn suggest_pipeline(&self, input_format: &str) -> Vec<&ForgeCrate> {
        // Find: detector → decomposer → transform → assembler
        let mut pipeline = Vec::new();
        if let Some(detector) = self.find("forge-detect") { pipeline.push(detector); }
        let decomposers = self.by_format(input_format);
        if let Some(first) = decomposers.first() { pipeline.push(first); }
        if let Some(conservation) = self.find("forge-conservation") { pipeline.push(conservation); }
        if let Some(transform) = self.find("forge-transform") { pipeline.push(transform); }
        if let Some(a2a) = self.find("forge-a2a") { pipeline.push(a2a); }
        if let Some(pipeline_crate) = self.find("forge-pipeline") { pipeline.push(pipeline_crate); }
        pipeline
    }

    fn builtin_crates() -> Vec<ForgeCrate> {
        vec![
            ForgeCrate { name: "forge-flux".into(), version: "0.1.0".into(), description: "Core tile decomposition engine".into(), domain: "core".into(), input_formats: vec!["text".into(),"csv".into(),"json".into(),"srt".into(),"code".into()], output_tile_kind: "Tile".into(), test_count: 50, url: "https://github.com/SuperInstance/forge-flux".into() },
            ForgeCrate { name: "forge-text".into(), version: "0.1.0".into(), description: "Text decomposition into tiles".into(), domain: "decompose".into(), input_formats: vec!["text".into(),"markdown".into()], output_tile_kind: "TextTile".into(), test_count: 19, url: "https://github.com/SuperInstance/forge-text".into() },
            ForgeCrate { name: "forge-data".into(), version: "0.1.0".into(), description: "Structured data decomposition (CSV/JSON/TSV)".into(), domain: "decompose".into(), input_formats: vec!["csv".into(),"json".into(),"tsv".into()], output_tile_kind: "DataTile".into(), test_count: 27, url: "https://github.com/SuperInstance/forge-data".into() },
            ForgeCrate { name: "forge-code".into(), version: "0.1.0".into(), description: "Code decomposition into tiles".into(), domain: "decompose".into(), input_formats: vec!["rust".into(),"python".into(),"typescript".into(),"go".into(),"c".into(),"java".into()], output_tile_kind: "CodeTile".into(), test_count: 15, url: "https://github.com/SuperInstance/forge-code".into() },
            ForgeCrate { name: "forge-audio".into(), version: "0.1.0".into(), description: "Audio decomposition into tiles".into(), domain: "decompose".into(), input_formats: vec!["audio".into(),"wav".into()], output_tile_kind: "AudioTile".into(), test_count: 14, url: "https://github.com/SuperInstance/forge-audio".into() },
            ForgeCrate { name: "forge-image".into(), version: "0.1.0".into(), description: "Image decomposition into tiles".into(), domain: "decompose".into(), input_formats: vec!["image".into(),"png".into(),"jpeg".into()], output_tile_kind: "ImageTile".into(), test_count: 14, url: "https://github.com/SuperInstance/forge-image".into() },
            ForgeCrate { name: "forge-sensor".into(), version: "0.1.0".into(), description: "Sensor data decomposition into tiles".into(), domain: "decompose".into(), input_formats: vec!["sensor".into()], output_tile_kind: "SensorTile".into(), test_count: 12, url: "https://github.com/SuperInstance/forge-sensor".into() },
            ForgeCrate { name: "forge-subtitle".into(), version: "0.1.0".into(), description: "Subtitle decomposition (SRT/VTT)".into(), domain: "decompose".into(), input_formats: vec!["srt".into(),"vtt".into()], output_tile_kind: "SubtitleTile".into(), test_count: 21, url: "https://github.com/SuperInstance/forge-subtitle".into() },
            ForgeCrate { name: "forge-soniqo".into(), version: "0.1.0".into(), description: "Audio spectral decomposition".into(), domain: "decompose".into(), input_formats: vec!["audio".into(),"wav".into()], output_tile_kind: "AudioTile".into(), test_count: 11, url: "https://github.com/SuperInstance/forge-soniqo".into() },
            ForgeCrate { name: "forge-memory".into(), version: "0.1.0".into(), description: "Tile memory store".into(), domain: "storage".into(), input_formats: vec![], output_tile_kind: "Tile".into(), test_count: 13, url: "https://github.com/SuperInstance/forge-memory".into() },
            ForgeCrate { name: "forge-conservation".into(), version: "0.1.0".into(), description: "Conservation ratio tracking".into(), domain: "analysis".into(), input_formats: vec![], output_tile_kind: "Report".into(), test_count: 21, url: "https://github.com/SuperInstance/forge-conservation".into() },
            ForgeCrate { name: "forge-transform".into(), version: "0.1.0".into(), description: "Tile transform library".into(), domain: "transform".into(), input_formats: vec![], output_tile_kind: "TileData".into(), test_count: 18, url: "https://github.com/SuperInstance/forge-transform".into() },
            ForgeCrate { name: "forge-pipeline".into(), version: "0.1.0".into(), description: "Pipeline orchestration".into(), domain: "orchestration".into(), input_formats: vec![], output_tile_kind: "StageOutput".into(), test_count: 18, url: "https://github.com/SuperInstance/forge-pipeline".into() },
            ForgeCrate { name: "forge-a2a".into(), version: "0.1.0".into(), description: "A2A messaging for tile pipelines".into(), domain: "messaging".into(), input_formats: vec![], output_tile_kind: "ForgeMessage".into(), test_count: 18, url: "https://github.com/SuperInstance/forge-a2a".into() },
            ForgeCrate { name: "forge-detect".into(), version: "0.1.0".into(), description: "Input format detection".into(), domain: "detection".into(), input_formats: vec![], output_tile_kind: "InputFormat".into(), test_count: 15, url: "https://github.com/SuperInstance/forge-detect".into() },
            ForgeCrate { name: "forge-tick".into(), version: "0.1.0".into(), description: "Tile-to-Tick conversion for Plato agents".into(), domain: "plato".into(), input_formats: vec![], output_tile_kind: "Tick".into(), test_count: 15, url: "https://github.com/SuperInstance/forge-tick".into() },
        ]
    }
}

impl Default for ForgeRegistry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let r = ForgeRegistry::new();
        assert!(r.all().len() >= 15);
    }

    #[test]
    fn test_total_tests() {
        let report = ForgeRegistry::new().ecosystem_report();
        assert!(report.total_tests >= 280);
    }

    #[test]
    fn test_by_domain() {
        let r = ForgeRegistry::new();
        let decomposers = r.by_domain("decompose");
        assert!(decomposers.len() >= 6);
    }

    #[test]
    fn test_by_format() {
        let r = ForgeRegistry::new();
        let csv = r.by_format("csv");
        assert!(csv.len() >= 2);
    }

    #[test]
    fn test_find() {
        let r = ForgeRegistry::new();
        let flux = r.find("forge-flux").unwrap();
        assert_eq!(flux.test_count, 50);
    }

    #[test]
    fn test_find_missing() {
        assert!(ForgeRegistry::new().find("nonexistent").is_none());
    }

    #[test]
    fn test_suggest_pipeline() {
        let reg = ForgeRegistry::new();
        let pipeline = reg.suggest_pipeline("csv");
        assert!(pipeline.len() >= 3);
    }

    #[test]
    fn test_ecosystem_report_domains() {
        let report = ForgeRegistry::new().ecosystem_report();
        assert!(report.domains.contains_key("decompose"));
        assert!(report.domains.contains_key("core"));
    }

    #[test]
    fn test_ecosystem_report_formats() {
        let report = ForgeRegistry::new().ecosystem_report();
        assert!(report.input_formats.contains(&"csv".to_string()));
        assert!(report.input_formats.contains(&"text".to_string()));
    }

    #[test]
    fn test_register_custom() {
        let mut r = ForgeRegistry::new();
        let initial = r.all().len();
        r.register(ForgeCrate {
            name: "forge-custom".into(), version: "0.1.0".into(),
            description: "Custom".into(), domain: "custom".into(),
            input_formats: vec!["xyz".into()], output_tile_kind: "Tile".into(),
            test_count: 5, url: "https://example.com".into(),
        });
        assert_eq!(r.all().len(), initial + 1);
    }

    #[test]
    fn test_report_serialization() {
        let report = ForgeRegistry::new().ecosystem_report();
        let json = serde_json::to_string(&report).unwrap();
        let back: EcosystemReport = serde_json::from_str(&json).unwrap();
        assert_eq!(report.total_crates, back.total_crates);
    }

    #[test]
    fn test_crate_serialization() {
        let r = ForgeRegistry::new();
        let flux = r.find("forge-flux").unwrap();
        let json = serde_json::to_string(flux).unwrap();
        let back: ForgeCrate = serde_json::from_str(&json).unwrap();
        assert_eq!(flux.name, back.name);
    }
}
