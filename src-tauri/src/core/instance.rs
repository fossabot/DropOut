use serde::{Deserialize, Serialize};
use std::fs::{self, symlink_metadata};
use std::io::{Read, Write};
use std::path::{PathBuf};
use uuid::Uuid;

/// Represents a game instance with its own isolated game directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub version_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub created_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_played: Option<i64>,
    
    // Instance-specific settings (override global if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_memory: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm_args: Option<String>,
}

impl Instance {
    pub fn new(name: String, version_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            version_id,
            icon: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            last_played: None,
            java_path: None,
            min_memory: None,
            max_memory: None,
            width: None,
            height: None,
            jvm_args: None,
        }
    }
}

/// Index file storing all instance references
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceIndex {
    pub instances: Vec<InstanceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_instance_id: Option<String>,
}

/// Reference to an instance (stored in index)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceRef {
    pub id: String,
    pub name: String,
    pub version_id: String,
}

impl From<&Instance> for InstanceRef {
    fn from(instance: &Instance) -> Self {
        Self {
            id: instance.id.clone(),
            name: instance.name.clone(),
            version_id: instance.version_id.clone(),
        }
    }
}

/// Manages game instances
pub struct InstanceManager {
    app_data_dir: PathBuf,
}

/// Validate instance ID to prevent path traversal attacks
fn validate_instance_id(instance_id: &str) -> Result<(), String> {
    // Only allow alphanumeric characters and hyphens (UUID format)
    if instance_id.is_empty() {
        return Err("Instance ID cannot be empty".to_string());
    }
    
    if !instance_id.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Instance ID contains invalid characters".to_string());
    }
    
    if instance_id.contains("..") {
        return Err("Instance ID contains path traversal attempt".to_string());
    }
    
    Ok(())
}

impl InstanceManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self { app_data_dir }
    }

    /// Get the instances directory
    fn instances_dir(&self) -> PathBuf {
        self.app_data_dir.join("instances")
    }

    /// Get the index file path
    fn index_path(&self) -> PathBuf {
        self.app_data_dir.join("instances.json")
    }

    /// Get an instance directory by ID (with validation)
    pub fn instance_dir(&self, instance_id: &str) -> Result<PathBuf, String> {
        validate_instance_id(instance_id)?;
        Ok(self.instances_dir().join(instance_id))
    }

    /// Load the instance index
    pub fn load_index(&self) -> Result<InstanceIndex, String> {
        let path = self.index_path();
        if !path.exists() {
            return Ok(InstanceIndex::default());
        }
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    /// Save the instance index
    fn save_index(&self, index: &InstanceIndex) -> Result<(), String> {
        let path = self.index_path();
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
        let content = serde_json::to_string_pretty(index).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())
    }

    /// Load an instance by ID
    pub fn load_instance(&self, instance_id: &str) -> Result<Instance, String> {
        let instance_dir = self.instance_dir(instance_id)?;
        let instance_json = instance_dir.join("instance.json");
        
        if !instance_json.exists() {
            return Err(format!("Instance {} not found", instance_id));
        }
        
        let content = fs::read_to_string(&instance_json).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    /// Save an instance
    fn save_instance(&self, instance: &Instance) -> Result<(), String> {
        let instance_dir = self.instance_dir(&instance.id)?;
        fs::create_dir_all(&instance_dir).map_err(|e| e.to_string())?;
        
        let instance_json = instance_dir.join("instance.json");
        let content = serde_json::to_string_pretty(instance).map_err(|e| e.to_string())?;
        fs::write(&instance_json, content).map_err(|e| e.to_string())
    }

    /// Create a new instance
    pub fn create_instance(&self, name: String, version_id: String) -> Result<Instance, String> {
        let instance = Instance::new(name, version_id);
        
        // Create instance directory structure
        let instance_dir = self.instance_dir(&instance.id)?;
        fs::create_dir_all(&instance_dir).map_err(|e| e.to_string())?;
        
        // Create subdirectories
        for subdir in &["mods", "saves", "resourcepacks", "shaderpacks", "config", "logs"] {
            fs::create_dir_all(instance_dir.join(subdir)).map_err(|e| e.to_string())?;
        }
        
        // Save instance.json
        self.save_instance(&instance)?;
        
        // Update index
        let mut index = self.load_index()?;
        index.instances.push(InstanceRef::from(&instance));
        
        // If this is the first instance, set it as active
        if index.active_instance_id.is_none() {
            index.active_instance_id = Some(instance.id.clone());
        }
        
        self.save_index(&index)?;
        
        Ok(instance)
    }

    /// Delete an instance
    pub fn delete_instance(&self, instance_id: &str) -> Result<(), String> {
        // Remove from index first
        let mut index = self.load_index()?;
        index.instances.retain(|i| i.id != instance_id);
        
        // If deleted instance was active, select another or none
        if index.active_instance_id.as_deref() == Some(instance_id) {
            index.active_instance_id = index.instances.first().map(|i| i.id.clone());
        }
        
        self.save_index(&index)?;
        
        // Delete instance directory
        let instance_dir = self.instance_dir(instance_id)?;
        if instance_dir.exists() {
            fs::remove_dir_all(&instance_dir).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }

    /// List all instances
    pub fn list_instances(&self) -> Result<Vec<Instance>, String> {
        let index = self.load_index()?;
        let mut instances = Vec::new();
        
        for instance_ref in &index.instances {
            match self.load_instance(&instance_ref.id) {
                Ok(instance) => instances.push(instance),
                Err(e) => {
                    eprintln!("Warning: Failed to load instance {}: {}", instance_ref.id, e);
                }
            }
        }
        
        Ok(instances)
    }

    /// Update an instance
    pub fn update_instance(&self, instance: Instance) -> Result<Instance, String> {
        // Verify instance exists
        let instance_dir = self.instance_dir(&instance.id)?;
        if !instance_dir.exists() {
            return Err(format!("Instance {} not found", instance.id));
        }
        
        // Save updated instance
        self.save_instance(&instance)?;
        
        // Update index (name/version might have changed)
        let mut index = self.load_index()?;
        if let Some(instance_ref) = index.instances.iter_mut().find(|i| i.id == instance.id) {
            instance_ref.name = instance.name.clone();
            instance_ref.version_id = instance.version_id.clone();
        }
        self.save_index(&index)?;
        
        Ok(instance)
    }

    /// Duplicate an instance
    pub fn duplicate_instance(&self, instance_id: &str, new_name: String) -> Result<Instance, String> {
        let source = self.load_instance(instance_id)?;
        let source_dir = self.instance_dir(instance_id)?;
        
        // Create new instance with same settings
        let mut new_instance = Instance::new(new_name, source.version_id.clone());
        new_instance.java_path = source.java_path.clone();
        new_instance.min_memory = source.min_memory;
        new_instance.max_memory = source.max_memory;
        new_instance.width = source.width;
        new_instance.height = source.height;
        new_instance.jvm_args = source.jvm_args.clone();
        
        let new_dir = self.instance_dir(&new_instance.id)?;
        
        // Copy directory contents (skip symlinks for security)
        copy_dir_recursive(&source_dir, &new_dir)?;
        
        // Save new instance.json (overwrites copied one)
        self.save_instance(&new_instance)?;
        
        // Update index
        let mut index = self.load_index()?;
        index.instances.push(InstanceRef::from(&new_instance));
        self.save_index(&index)?;
        
        Ok(new_instance)
    }

    /// Update last played time
    pub fn update_last_played(&self, instance_id: &str) -> Result<(), String> {
        let mut instance = self.load_instance(instance_id)?;
        instance.last_played = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        );
        self.save_instance(&instance)
    }

    /// Set active instance
    pub fn set_active_instance(&self, instance_id: Option<String>) -> Result<(), String> {
        let mut index = self.load_index()?;
        
        // Verify instance exists if setting one
        if let Some(ref id) = instance_id {
            if !index.instances.iter().any(|i| &i.id == id) {
                return Err(format!("Instance {} not found", id));
            }
        }
        
        index.active_instance_id = instance_id;
        self.save_index(&index)
    }

    /// Get active instance ID
    pub fn get_active_instance_id(&self) -> Result<Option<String>, String> {
        let index = self.load_index()?;
        Ok(index.active_instance_id)
    }

    /// Export instance to zip file
    pub async fn export_instance(&self, instance_id: &str, output_path: PathBuf) -> Result<(), String> {
        let instance_dir = self.instance_dir(instance_id)?;
        if !instance_dir.exists() {
            return Err(format!("Instance {} not found", instance_id));
        }
        
        // Create zip file
        let file = std::fs::File::create(&output_path).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        
        // Walk directory and add files (skip symlinks for security)
        add_dir_to_zip(&mut zip, &instance_dir, &instance_dir, options)?;
        
        zip.finish().map_err(|e| e.to_string())?;
        
        Ok(())
    }

    /// Import instance from zip file (with security validation)
    pub async fn import_instance(&self, zip_path: PathBuf, name: Option<String>) -> Result<Instance, String> {
        let file = std::fs::File::open(&zip_path).map_err(|e| e.to_string())?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        // Create new instance ID
        let new_id = Uuid::new_v4().to_string();
        let instance_dir = self.instance_dir(&new_id)?;
        fs::create_dir_all(&instance_dir).map_err(|e| e.to_string())?;
        
        // Securely extract zip - validate each file path to prevent Zip Slip attacks
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            
            // Use enclosed_name() for security - it returns None for paths with .. or absolute paths
            let outpath = match file.enclosed_name() {
                Some(path) => instance_dir.join(path),
                None => {
                    eprintln!("Skipping potentially unsafe path in zip: {:?}", file.name());
                    continue;
                }
            };
            
            // Double-check that the resolved path is still within instance_dir
            if !outpath.starts_with(&instance_dir) {
                return Err(format!(
                    "Zip file contains path traversal attempt: {}",
                    file.name()
                ));
            }
            
            if file.is_dir() {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            } else {
                // Create parent directories if needed
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                
                // Write file contents
                let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                outfile.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }
        
        // Load and update instance.json
        let instance_json_path = instance_dir.join("instance.json");
        let mut instance: Instance = if instance_json_path.exists() {
            let content = fs::read_to_string(&instance_json_path).map_err(|e| e.to_string())?;
            serde_json::from_str(&content).map_err(|e| e.to_string())?
        } else {
            // Create default instance if no instance.json found
            Instance::new(
                name.clone().unwrap_or_else(|| "Imported Instance".to_string()),
                "1.20.4".to_string(), // Default version, should be updated
            )
        };
        
        // Update instance with new ID and optionally new name
        instance.id = new_id;
        if let Some(new_name) = name {
            instance.name = new_name;
        }
        instance.created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        instance.last_played = None;
        
        // Save updated instance
        self.save_instance(&instance)?;
        
        // Update index
        let mut index = self.load_index()?;
        index.instances.push(InstanceRef::from(&instance));
        self.save_index(&index)?;
        
        Ok(instance)
    }
}

/// Recursively copy a directory (skips symlinks for security)
fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        
        // Check if it's a symlink - skip for security
        let metadata = symlink_metadata(&path).map_err(|e| e.to_string())?;
        if metadata.is_symlink() {
            eprintln!("Skipping symlink during copy: {:?}", path);
            continue;
        }
        
        if metadata.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path).map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

/// Add directory contents to zip (skips symlinks for security)
fn add_dir_to_zip<W: std::io::Write + std::io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    dir: &PathBuf,
    base: &PathBuf,
    options: zip::write::SimpleFileOptions,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        // Check if it's a symlink - skip for security
        let metadata = symlink_metadata(&path).map_err(|e| e.to_string())?;
        if metadata.is_symlink() {
            eprintln!("Skipping symlink during zip: {:?}", path);
            continue;
        }
        
        let relative_path = path.strip_prefix(base).map_err(|e| e.to_string())?;
        let name = relative_path.to_string_lossy();
        
        if metadata.is_dir() {
            zip.add_directory(format!("{}/", name), options)
                .map_err(|e| e.to_string())?;
            add_dir_to_zip(zip, &path, base, options)?;
        } else {
            zip.start_file(name.to_string(), options)
                .map_err(|e| e.to_string())?;
            let content = fs::read(&path).map_err(|e| e.to_string())?;
            std::io::Write::write_all(zip, &content).map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}
