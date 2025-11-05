use crate::cli::Framework;
use crate::error::DetectionError;
use std::path::Path;
use std::collections::HashMap;

/// Trait for detecting web frameworks in Rust projects
pub trait FrameworkDetector: Send + Sync {
    /// Detect if this framework is present in the project
    fn detect(&self, project_path: &Path) -> Result<bool, DetectionError>;
    
    /// Get the framework this detector is responsible for
    fn framework(&self) -> Framework;
    
    /// Get the confidence score (0.0 to 1.0) of the detection
    fn confidence(&self, project_path: &Path) -> f32;
}

/// Information about a detected project
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub framework: Framework,
    pub confidence: f32,
    pub dependencies: Vec<String>,
    pub project_name: String,
    pub project_structure: ProjectStructure,
}

/// Represents the structure of a Rust project
#[derive(Debug, Clone)]
pub struct ProjectStructure {
    pub has_src_dir: bool,
    pub has_lib_rs: bool,
    pub has_main_rs: bool,
    pub module_dirs: Vec<String>,
    pub common_patterns: Vec<String>,
}

/// Parsed Cargo.toml information
#[derive(Debug, Clone)]
struct CargoManifest {
    project_name: String,
    dependencies: HashMap<String, DependencyInfo>,
    dev_dependencies: HashMap<String, DependencyInfo>,
}

/// Information about a dependency
#[derive(Debug, Clone)]
struct DependencyInfo {
    version: Option<String>,
    features: Vec<String>,
}

/// Orchestrates framework detection using multiple detectors
pub struct ProjectAnalyzer {
    detectors: Vec<Box<dyn FrameworkDetector>>,
}

impl ProjectAnalyzer {
    /// Create a new ProjectAnalyzer with all built-in detectors
    pub fn new() -> Self {
        let detectors: Vec<Box<dyn FrameworkDetector>> = vec![
            Box::new(AxumDetector),
            Box::new(ActixWebDetector),
            Box::new(WarpDetector),
            Box::new(RocketDetector),
        ];
        
        Self { detectors }
    }
    
    /// Analyze a project and detect its web framework
    pub fn analyze_project(&self, project_path: &Path) -> Result<ProjectInfo, DetectionError> {
        // Parse Cargo.toml
        let manifest = self.parse_cargo_toml(project_path)?;
        
        // Analyze project structure
        let project_structure = self.analyze_project_structure(project_path)?;
        
        // Run all detectors and collect results
        let mut detection_results: Vec<(Framework, f32)> = Vec::new();
        
        for detector in &self.detectors {
            if detector.detect(project_path)? {
                let confidence = detector.confidence(project_path);
                detection_results.push((detector.framework(), confidence));
            }
        }
        
        // If no framework detected, try fallback detection
        if detection_results.is_empty() {
            if let Some((framework, confidence)) = self.fallback_detection(project_path, &manifest, &project_structure)? {
                detection_results.push((framework, confidence));
            }
        }
        
        // If still no framework detected, return error
        if detection_results.is_empty() {
            return Err(DetectionError::NoFrameworkDetected);
        }
        
        // If multiple frameworks detected with similar confidence, return error
        if detection_results.len() > 1 {
            let max_confidence = detection_results.iter()
                .map(|(_, conf)| *conf)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0);
            
            let high_confidence_frameworks: Vec<String> = detection_results.iter()
                .filter(|(_, conf)| (*conf - max_confidence).abs() < 0.1)
                .map(|(fw, _)| format!("{:?}", fw))
                .collect();
            
            if high_confidence_frameworks.len() > 1 {
                return Err(DetectionError::MultipleFrameworks(high_confidence_frameworks));
            }
        }
        
        // Return the framework with highest confidence
        let (framework, confidence) = detection_results.into_iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        
        // Collect all dependency names
        let dependencies: Vec<String> = manifest.dependencies.keys()
            .chain(manifest.dev_dependencies.keys())
            .cloned()
            .collect();
        
        Ok(ProjectInfo {
            framework,
            confidence,
            dependencies,
            project_name: manifest.project_name,
            project_structure,
        })
    }
    
    /// Parse Cargo.toml and extract dependency information
    fn parse_cargo_toml(&self, project_path: &Path) -> Result<CargoManifest, DetectionError> {
        let cargo_toml_path = project_path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return Err(DetectionError::CargoTomlError(
                "Cargo.toml not found in project directory".to_string()
            ));
        }
        
        let cargo_content = std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        let cargo_toml: toml::Value = toml::from_str(&cargo_content)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        let project_name = cargo_toml
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("unknown")
            .to_string();
        
        let dependencies = self.parse_dependencies(cargo_toml.get("dependencies"));
        let dev_dependencies = self.parse_dependencies(cargo_toml.get("dev-dependencies"));
        
        Ok(CargoManifest {
            project_name,
            dependencies,
            dev_dependencies,
        })
    }
    
    /// Parse dependencies section from Cargo.toml
    fn parse_dependencies(&self, deps_value: Option<&toml::Value>) -> HashMap<String, DependencyInfo> {
        let mut deps = HashMap::new();
        
        if let Some(dependencies) = deps_value.and_then(|d| d.as_table()) {
            for (name, value) in dependencies {
                let dep_info = match value {
                    toml::Value::String(version) => DependencyInfo {
                        version: Some(version.clone()),
                        features: Vec::new(),
                    },
                    toml::Value::Table(table) => {
                        let version = table.get("version")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        
                        let features = table.get("features")
                            .and_then(|f| f.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        
                        DependencyInfo { version, features }
                    },
                    _ => DependencyInfo {
                        version: None,
                        features: Vec::new(),
                    },
                };
                
                deps.insert(name.clone(), dep_info);
            }
        }
        
        deps
    }
    
    /// Analyze the project's file structure
    fn analyze_project_structure(&self, project_path: &Path) -> Result<ProjectStructure, DetectionError> {
        let src_path = project_path.join("src");
        let has_src_dir = src_path.exists() && src_path.is_dir();
        
        let has_lib_rs = src_path.join("lib.rs").exists();
        let has_main_rs = src_path.join("main.rs").exists();
        
        let mut module_dirs = Vec::new();
        let mut common_patterns = Vec::new();
        
        if has_src_dir {
            if let Ok(entries) = std::fs::read_dir(&src_path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(dir_name) = entry.file_name().to_str() {
                            module_dirs.push(dir_name.to_string());
                            
                            // Check for common web framework patterns
                            match dir_name {
                                "routes" | "handlers" | "controllers" => {
                                    common_patterns.push("route_structure".to_string());
                                },
                                "services" | "business" => {
                                    common_patterns.push("service_layer".to_string());
                                },
                                "middleware" | "guards" => {
                                    common_patterns.push("middleware_pattern".to_string());
                                },
                                "models" | "entities" => {
                                    common_patterns.push("data_models".to_string());
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        
        Ok(ProjectStructure {
            has_src_dir,
            has_lib_rs,
            has_main_rs,
            module_dirs,
            common_patterns,
        })
    }
    
    /// Fallback detection using common import patterns in source files
    fn fallback_detection(
        &self,
        project_path: &Path,
        manifest: &CargoManifest,
        structure: &ProjectStructure,
    ) -> Result<Option<(Framework, f32)>, DetectionError> {
        // Check for framework-specific patterns in source files
        let src_path = project_path.join("src");
        if !src_path.exists() {
            return Ok(None);
        }
        
        let mut framework_scores: HashMap<Framework, f32> = HashMap::new();
        
        // Scan all .rs files for import patterns
        self.scan_directory_for_patterns(&src_path, &mut framework_scores)?;
        
        // Boost scores based on dependency presence
        for (dep_name, _) in &manifest.dependencies {
            match dep_name.as_str() {
                "axum" => *framework_scores.entry(Framework::Axum).or_insert(0.0) += 0.4,
                "actix-web" => *framework_scores.entry(Framework::ActixWeb).or_insert(0.0) += 0.4,
                "warp" => *framework_scores.entry(Framework::Warp).or_insert(0.0) += 0.4,
                "rocket" => *framework_scores.entry(Framework::Rocket).or_insert(0.0) += 0.4,
                "tower" | "tower-http" => *framework_scores.entry(Framework::Axum).or_insert(0.0) += 0.1,
                "actix-rt" => *framework_scores.entry(Framework::ActixWeb).or_insert(0.0) += 0.1,
                _ => {}
            }
        }
        
        // Boost scores based on project structure patterns
        if structure.common_patterns.contains(&"route_structure".to_string()) {
            for score in framework_scores.values_mut() {
                *score += 0.05;
            }
        }
        
        // Find the framework with the highest score
        framework_scores.into_iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(fw, score)| {
                // Only return if confidence is reasonable
                if score >= 0.3 {
                    Some((fw, score.min(1.0)))
                } else {
                    None
                }
            })
            .unwrap_or(None)
            .map_or(Ok(None), |result| Ok(Some(result)))
    }
    
    /// Recursively scan directory for framework-specific import patterns
    fn scan_directory_for_patterns(
        &self,
        dir_path: &Path,
        scores: &mut HashMap<Framework, f32>,
    ) -> Result<(), DetectionError> {
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    // Recursively scan subdirectories
                    self.scan_directory_for_patterns(&path, scores)?;
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    // Scan Rust source files
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        self.analyze_source_patterns(&content, scores);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Analyze source code for framework-specific patterns
    fn analyze_source_patterns(&self, content: &str, scores: &mut HashMap<Framework, f32>) {
        // Axum patterns
        if content.contains("use axum::") || content.contains("axum::Router") 
            || content.contains("axum::extract::") || content.contains("axum::response::") {
            *scores.entry(Framework::Axum).or_insert(0.0) += 0.15;
        }
        
        // Actix-web patterns
        if content.contains("use actix_web::") || content.contains("actix_web::")
            || content.contains("HttpServer::new") || content.contains("web::Json")
            || content.contains("web::Path") || content.contains("HttpResponse::") {
            *scores.entry(Framework::ActixWeb).or_insert(0.0) += 0.15;
        }
        
        // Warp patterns
        if content.contains("use warp::") || content.contains("warp::Filter")
            || content.contains("warp::reply") || content.contains("warp::path") {
            *scores.entry(Framework::Warp).or_insert(0.0) += 0.15;
        }
        
        // Rocket patterns
        if content.contains("use rocket::") || content.contains("#[get(")
            || content.contains("#[post(") || content.contains("#[put(")
            || content.contains("#[delete(") || content.contains("rocket::launch") {
            *scores.entry(Framework::Rocket).or_insert(0.0) += 0.15;
        }
    }
}

impl Default for ProjectAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Detector for Axum framework
struct AxumDetector;

impl FrameworkDetector for AxumDetector {
    fn detect(&self, project_path: &Path) -> Result<bool, DetectionError> {
        let cargo_toml_path = project_path.join("Cargo.toml");
        let cargo_content = std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        let cargo_toml: toml::Value = toml::from_str(&cargo_content)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        // Check for axum dependency in dependencies
        let has_axum = cargo_toml
            .get("dependencies")
            .and_then(|d| d.as_table())
            .map(|deps| deps.contains_key("axum"))
            .unwrap_or(false);
        
        Ok(has_axum)
    }
    
    fn framework(&self) -> Framework {
        Framework::Axum
    }
    
    fn confidence(&self, project_path: &Path) -> f32 {
        let mut confidence: f32 = 0.0;
        
        // Parse Cargo.toml properly
        if let Ok(cargo_content) = std::fs::read_to_string(project_path.join("Cargo.toml")) {
            if let Ok(cargo_toml) = toml::from_str::<toml::Value>(&cargo_content) {
                if let Some(deps) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
                    if deps.contains_key("axum") {
                        confidence += 0.5;
                    }
                    if deps.contains_key("tower") || deps.contains_key("tower-http") {
                        confidence += 0.15;
                    }
                    if deps.contains_key("hyper") {
                        confidence += 0.1;
                    }
                    if deps.contains_key("tokio") {
                        confidence += 0.05;
                    }
                }
            }
        }
        
        // Check for common Axum patterns in source files
        let src_path = project_path.join("src");
        if src_path.exists() {
            confidence += self.scan_for_axum_patterns(&src_path);
        }
        
        confidence.min(1.0)
    }
}

impl AxumDetector {
    fn scan_for_axum_patterns(&self, dir: &Path) -> f32 {
        let mut score = 0.0;
        let mut files_checked = 0;
        
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    score += self.scan_for_axum_patterns(&path);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        files_checked += 1;
                        
                        if content.contains("axum::Router") {
                            score += 0.2;
                        }
                        if content.contains("axum::extract::") {
                            score += 0.1;
                        }
                        if content.contains("axum::response::") {
                            score += 0.05;
                        }
                        if content.contains("use axum::") {
                            score += 0.05;
                        }
                    }
                }
                
                // Limit scanning to avoid performance issues
                if files_checked >= 10 {
                    break;
                }
            }
        }
        
        score.min(0.3)
    }
}

/// Detector for Actix-web framework
struct ActixWebDetector;

impl FrameworkDetector for ActixWebDetector {
    fn detect(&self, project_path: &Path) -> Result<bool, DetectionError> {
        let cargo_toml_path = project_path.join("Cargo.toml");
        let cargo_content = std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        let cargo_toml: toml::Value = toml::from_str(&cargo_content)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        // Check for actix-web dependency
        let has_actix = cargo_toml
            .get("dependencies")
            .and_then(|d| d.as_table())
            .map(|deps| deps.contains_key("actix-web"))
            .unwrap_or(false);
        
        Ok(has_actix)
    }
    
    fn framework(&self) -> Framework {
        Framework::ActixWeb
    }
    
    fn confidence(&self, project_path: &Path) -> f32 {
        let mut confidence: f32 = 0.0;
        
        // Parse Cargo.toml properly
        if let Ok(cargo_content) = std::fs::read_to_string(project_path.join("Cargo.toml")) {
            if let Ok(cargo_toml) = toml::from_str::<toml::Value>(&cargo_content) {
                if let Some(deps) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
                    if deps.contains_key("actix-web") {
                        confidence += 0.5;
                    }
                    if deps.contains_key("actix-rt") {
                        confidence += 0.15;
                    }
                    if deps.contains_key("actix-files") || deps.contains_key("actix-cors") {
                        confidence += 0.1;
                    }
                }
            }
        }
        
        // Check for common Actix-web patterns in source files
        let src_path = project_path.join("src");
        if src_path.exists() {
            confidence += self.scan_for_actix_patterns(&src_path);
        }
        
        confidence.min(1.0)
    }
}

impl ActixWebDetector {
    fn scan_for_actix_patterns(&self, dir: &Path) -> f32 {
        let mut score = 0.0;
        let mut files_checked = 0;
        
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    score += self.scan_for_actix_patterns(&path);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        files_checked += 1;
                        
                        if content.contains("HttpServer::new") {
                            score += 0.2;
                        }
                        if content.contains("actix_web::") {
                            score += 0.1;
                        }
                        if content.contains("web::Json") || content.contains("web::Path") {
                            score += 0.05;
                        }
                        if content.contains("HttpResponse::") {
                            score += 0.05;
                        }
                    }
                }
                
                if files_checked >= 10 {
                    break;
                }
            }
        }
        
        score.min(0.3)
    }
}

/// Detector for Warp framework
struct WarpDetector;

impl FrameworkDetector for WarpDetector {
    fn detect(&self, project_path: &Path) -> Result<bool, DetectionError> {
        let cargo_toml_path = project_path.join("Cargo.toml");
        let cargo_content = std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        let cargo_toml: toml::Value = toml::from_str(&cargo_content)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        // Check for warp dependency
        let has_warp = cargo_toml
            .get("dependencies")
            .and_then(|d| d.as_table())
            .map(|deps| deps.contains_key("warp"))
            .unwrap_or(false);
        
        Ok(has_warp)
    }
    
    fn framework(&self) -> Framework {
        Framework::Warp
    }
    
    fn confidence(&self, project_path: &Path) -> f32 {
        let mut confidence: f32 = 0.0;
        
        // Parse Cargo.toml properly
        if let Ok(cargo_content) = std::fs::read_to_string(project_path.join("Cargo.toml")) {
            if let Ok(cargo_toml) = toml::from_str::<toml::Value>(&cargo_content) {
                if let Some(deps) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
                    if deps.contains_key("warp") {
                        confidence += 0.6;
                    }
                    if deps.contains_key("tokio") {
                        confidence += 0.05;
                    }
                }
            }
        }
        
        // Check for common Warp patterns in source files
        let src_path = project_path.join("src");
        if src_path.exists() {
            confidence += self.scan_for_warp_patterns(&src_path);
        }
        
        confidence.min(1.0)
    }
}

impl WarpDetector {
    fn scan_for_warp_patterns(&self, dir: &Path) -> f32 {
        let mut score = 0.0;
        let mut files_checked = 0;
        
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    score += self.scan_for_warp_patterns(&path);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        files_checked += 1;
                        
                        if content.contains("warp::Filter") {
                            score += 0.2;
                        }
                        if content.contains("warp::reply") {
                            score += 0.1;
                        }
                        if content.contains("warp::path") {
                            score += 0.05;
                        }
                        if content.contains("use warp::") {
                            score += 0.05;
                        }
                    }
                }
                
                if files_checked >= 10 {
                    break;
                }
            }
        }
        
        score.min(0.3)
    }
}

/// Detector for Rocket framework
struct RocketDetector;

impl FrameworkDetector for RocketDetector {
    fn detect(&self, project_path: &Path) -> Result<bool, DetectionError> {
        let cargo_toml_path = project_path.join("Cargo.toml");
        let cargo_content = std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        let cargo_toml: toml::Value = toml::from_str(&cargo_content)
            .map_err(|e| DetectionError::CargoTomlError(e.to_string()))?;
        
        // Check for rocket dependency
        let has_rocket = cargo_toml
            .get("dependencies")
            .and_then(|d| d.as_table())
            .map(|deps| deps.contains_key("rocket"))
            .unwrap_or(false);
        
        Ok(has_rocket)
    }
    
    fn framework(&self) -> Framework {
        Framework::Rocket
    }
    
    fn confidence(&self, project_path: &Path) -> f32 {
        let mut confidence: f32 = 0.0;
        
        // Parse Cargo.toml properly
        if let Ok(cargo_content) = std::fs::read_to_string(project_path.join("Cargo.toml")) {
            if let Ok(cargo_toml) = toml::from_str::<toml::Value>(&cargo_content) {
                if let Some(deps) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
                    if deps.contains_key("rocket") {
                        confidence += 0.6;
                    }
                    if deps.contains_key("rocket_contrib") {
                        confidence += 0.1;
                    }
                }
            }
        }
        
        // Check for common Rocket patterns in source files
        let src_path = project_path.join("src");
        if src_path.exists() {
            confidence += self.scan_for_rocket_patterns(&src_path);
        }
        
        confidence.min(1.0)
    }
}

impl RocketDetector {
    fn scan_for_rocket_patterns(&self, dir: &Path) -> f32 {
        let mut score = 0.0;
        let mut files_checked = 0;
        
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    score += self.scan_for_rocket_patterns(&path);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        files_checked += 1;
                        
                        if content.contains("#[get(") || content.contains("#[post(") 
                            || content.contains("#[put(") || content.contains("#[delete(") {
                            score += 0.2;
                        }
                        if content.contains("rocket::launch") || content.contains("#[launch]") {
                            score += 0.15;
                        }
                        if content.contains("use rocket::") {
                            score += 0.05;
                        }
                    }
                }
                
                if files_checked >= 10 {
                    break;
                }
            }
        }
        
        score.min(0.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_project(temp_dir: &Path, cargo_toml_content: &str) -> std::io::Result<()> {
        fs::write(temp_dir.join("Cargo.toml"), cargo_toml_content)?;
        fs::create_dir_all(temp_dir.join("src"))?;
        fs::write(temp_dir.join("src/main.rs"), "fn main() {}")?;
        Ok(())
    }

    #[test]
    fn test_axum_detection() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
"#;
        create_test_project(temp_dir.path(), cargo_toml).unwrap();
        
        let analyzer = ProjectAnalyzer::new();
        let result = analyzer.analyze_project(temp_dir.path()).unwrap();
        
        assert_eq!(result.framework, Framework::Axum);
        assert!(result.confidence > 0.0);
        assert_eq!(result.project_name, "test-project");
        assert!(result.dependencies.contains(&"axum".to_string()));
    }

    #[test]
    fn test_actix_web_detection() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
actix-web = "4.0"
"#;
        create_test_project(temp_dir.path(), cargo_toml).unwrap();
        
        let analyzer = ProjectAnalyzer::new();
        let result = analyzer.analyze_project(temp_dir.path()).unwrap();
        
        assert_eq!(result.framework, Framework::ActixWeb);
        assert!(result.confidence > 0.0);
        assert!(result.dependencies.contains(&"actix-web".to_string()));
    }

    #[test]
    fn test_no_framework_detection() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
        create_test_project(temp_dir.path(), cargo_toml).unwrap();
        
        let analyzer = ProjectAnalyzer::new();
        let result = analyzer.analyze_project(temp_dir.path());
        
        assert!(matches!(result, Err(DetectionError::NoFrameworkDetected)));
    }
    
    #[test]
    fn test_project_structure_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
axum = "0.7"
"#;
        create_test_project(temp_dir.path(), cargo_toml).unwrap();
        
        // Create some module directories
        fs::create_dir_all(temp_dir.path().join("src/routes")).unwrap();
        fs::create_dir_all(temp_dir.path().join("src/services")).unwrap();
        
        let analyzer = ProjectAnalyzer::new();
        let result = analyzer.analyze_project(temp_dir.path()).unwrap();
        
        assert!(result.project_structure.has_src_dir);
        assert!(result.project_structure.has_main_rs);
        assert!(result.project_structure.module_dirs.contains(&"routes".to_string()));
        assert!(result.project_structure.module_dirs.contains(&"services".to_string()));
    }
    
    #[test]
    fn test_fallback_detection_with_imports() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
axum = "0.7"
"#;
        create_test_project(temp_dir.path(), cargo_toml).unwrap();
        
        // Add a file with Axum imports
        let main_content = r#"
use axum::Router;
use axum::extract::State;

fn main() {
    let app = Router::new();
}
"#;
        fs::write(temp_dir.path().join("src/main.rs"), main_content).unwrap();
        
        let analyzer = ProjectAnalyzer::new();
        let result = analyzer.analyze_project(temp_dir.path()).unwrap();
        
        assert_eq!(result.framework, Framework::Axum);
        // Confidence should be higher due to import patterns
        assert!(result.confidence > 0.5);
    }
    
    #[test]
    fn test_dependency_parsing_with_features() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"

[dependencies]
tokio = { version = "1.0", features = ["full", "macros"] }
axum = { version = "0.7", features = ["macros"] }
"#;
        create_test_project(temp_dir.path(), cargo_toml).unwrap();
        
        let analyzer = ProjectAnalyzer::new();
        let result = analyzer.analyze_project(temp_dir.path()).unwrap();
        
        assert_eq!(result.framework, Framework::Axum);
        assert!(result.dependencies.contains(&"tokio".to_string()));
        assert!(result.dependencies.contains(&"axum".to_string()));
    }
}
