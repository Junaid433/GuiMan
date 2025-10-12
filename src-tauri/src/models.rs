use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub repo: String,
    pub description: String,
    pub installed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct PackageDependencyInfo {
    pub dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub version: String,
    pub repository: String,
    pub installed: bool,
}

impl CommandResult {
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_data(message: String, data: serde_json::Value) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
        }
    }
}

