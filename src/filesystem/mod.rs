use std::path::{Path, PathBuf};
use crate::error::FileSystemError;

pub struct FileSystemManager {
    project_root: PathBuf,
    dry_run: bool,
}

impl FileSystemManager {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            dry_run: false,
        }
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub async fn create_file(&self, path: &Path, content: &str) -> Result<(), FileSystemError> {
        // Validate path is within project root
        self.validate_path(path)?;

        if path.exists() {
            return Err(FileSystemError::FileExists(path.display().to_string()));
        }

        if self.dry_run {
            println!("DRY RUN: Would create file: {}", path.display());
            return Ok(());
        }

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| FileSystemError::IoError(e.to_string()))?;
        }

        // Write file atomically
        tokio::fs::write(path, content).await
            .map_err(|e| FileSystemError::IoError(e.to_string()))?;

        Ok(())
    }

    pub async fn update_module_file(&self, module_path: &Path, new_export: &str) -> Result<(), FileSystemError> {
        self.validate_path(module_path)?;

        if self.dry_run {
            println!("DRY RUN: Would update module file: {} with export: {}", module_path.display(), new_export);
            return Ok(());
        }

        let mod_file = module_path.join("mod.rs");
        
        let content = if mod_file.exists() {
            tokio::fs::read_to_string(&mod_file).await
                .map_err(|e| FileSystemError::IoError(e.to_string()))?
        } else {
            String::new()
        };

        // Check if export already exists
        if content.contains(&format!("pub mod {};", new_export)) {
            return Ok(());
        }

        let updated_content = if content.is_empty() {
            format!("pub mod {};\n", new_export)
        } else {
            format!("{}\npub mod {};\n", content.trim(), new_export)
        };

        tokio::fs::write(&mod_file, updated_content).await
            .map_err(|e| FileSystemError::IoError(e.to_string()))?;

        Ok(())
    }

    pub fn validate_project_structure(&self) -> Result<(), FileSystemError> {
        let cargo_toml = self.project_root.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(FileSystemError::InvalidPath(
                "Not a valid Rust project: Cargo.toml not found".to_string()
            ));
        }

        let src_dir = self.project_root.join("src");
        if !src_dir.exists() {
            return Err(FileSystemError::DirectoryNotFound(
                "src directory not found".to_string()
            ));
        }

        Ok(())
    }

    fn validate_path(&self, path: &Path) -> Result<(), FileSystemError> {
        // Ensure path is absolute or make it relative to project root
        let full_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.project_root.join(path)
        };

        // Check for path traversal attacks
        if let Ok(canonical) = full_path.canonicalize() {
            if let Ok(project_canonical) = self.project_root.canonicalize() {
                if !canonical.starts_with(project_canonical) {
                    return Err(FileSystemError::PathTraversal(
                        path.display().to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}