use serde::{Deserialize, Serialize};
use std::process::Command;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DependencyNode {
    pub name: String,
    pub version: String,
    pub repo: String,
    pub installed: bool,
    pub dependencies: Vec<String>,
    pub optional_deps: Vec<String>,
    pub required_by: Vec<String>,
    pub level: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub root: String,
    pub nodes: HashMap<String, DependencyNode>,
    pub max_depth: usize,
}

/// Get dependency tree for a package
#[tauri::command]
pub async fn get_dependency_tree(package: String, max_depth: Option<usize>) -> Result<DependencyGraph, String> {
    println!("Getting dependency tree for: {}", package);
    let max_depth = max_depth.unwrap_or(2); // Reduced default depth
    let mut graph = DependencyGraph {
        root: package.clone(),
        nodes: HashMap::new(),
        max_depth,
    };
    
    let mut visited = HashSet::new();
    match build_dependency_tree(&package, 0, max_depth, &mut graph.nodes, &mut visited) {
        Ok(_) => {
            println!("Successfully built dependency tree with {} nodes", graph.nodes.len());
            Ok(graph)
        }
        Err(e) => {
            println!("Error building dependency tree: {}", e);
            Err(e)
        }
    }
}

/// Get reverse dependency tree (what depends on this package)
#[tauri::command]
pub async fn get_reverse_dependency_tree(package: String, max_depth: Option<usize>) -> Result<DependencyGraph, String> {
    let max_depth = max_depth.unwrap_or(2); // Reduced default depth
    let mut graph = DependencyGraph {
        root: package.clone(),
        nodes: HashMap::new(),
        max_depth,
    };
    
    let mut visited = HashSet::new();
    build_reverse_dependency_tree(&package, 0, max_depth, &mut graph.nodes, &mut visited)?;
    
    Ok(graph)
}

fn build_dependency_tree(
    package: &str,
    level: usize,
    max_depth: usize,
    nodes: &mut HashMap<String, DependencyNode>,
    visited: &mut HashSet<String>,
) -> Result<(), String> {
    // Prevent runaway processing
    if level > max_depth || visited.contains(package) || nodes.len() > 50 {
        return Ok(());
    }
    
    visited.insert(package.to_string());
    
    // Try to get package info - first from installed packages, then from repos, then from AUR
    let (dependencies, optional_deps, version, repo, installed) = 
        get_package_info_for_deps(package)?;
    
    // Get what requires this package (only works for installed packages)
    let required_by = get_required_by(package).unwrap_or_default();
    
    let node = DependencyNode {
        name: package.to_string(),
        version,
        repo,
        installed,
        dependencies: dependencies.clone(),
        optional_deps: optional_deps.clone(),
        required_by,
        level,
    };
    
    nodes.insert(package.to_string(), node);
    
    // Recursively build dependency tree
    for dep in dependencies {
        let dep_name = clean_dependency_name(&dep);
        if let Err(e) = build_dependency_tree(&dep_name, level + 1, max_depth, nodes, visited) {
            println!("Warning: Failed to build dependency tree for {}: {}", dep_name, e);
            // Continue with other dependencies instead of failing completely
        }
    }
    
    Ok(())
}

fn build_reverse_dependency_tree(
    package: &str,
    level: usize,
    max_depth: usize,
    nodes: &mut HashMap<String, DependencyNode>,
    visited: &mut HashSet<String>,
) -> Result<(), String> {
    // Prevent runaway processing
    if level > max_depth || visited.contains(package) || nodes.len() > 50 {
        return Ok(());
    }
    
    visited.insert(package.to_string());
    
    // Try to get package info - first from installed packages, then from repos, then from AUR
    let (dependencies, optional_deps, version, repo, installed) = 
        get_package_info_for_deps(package)?;
    
    let required_by = get_required_by(package).unwrap_or_default();
    
    let node = DependencyNode {
        name: package.to_string(),
        version,
        repo,
        installed,
        dependencies,
        optional_deps,
        required_by: required_by.clone(),
        level,
    };
    
    nodes.insert(package.to_string(), node);
    
    // Recursively build reverse dependency tree
    for req_by in required_by {
        if let Err(e) = build_reverse_dependency_tree(&req_by, level + 1, max_depth, nodes, visited) {
            println!("Warning: Failed to build reverse dependency tree for {}: {}", req_by, e);
            // Continue with other dependencies instead of failing completely
        }
    }
    
    Ok(())
}

fn extract_dependencies(output: &str) -> Vec<String> {
    for line in output.lines() {
        if line.starts_with("Depends On") {
            let deps_str = line.split(':').nth(1).unwrap_or("").trim();
            if deps_str == "None" {
                return Vec::new();
            }
            return deps_str
                .split_whitespace()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .filter(|s| !is_library_dependency(s)) // Filter out .so libraries
                .collect();
        }
    }
    Vec::new()
}

fn is_library_dependency(dep: &str) -> bool {
    // Filter out shared library dependencies that are not useful for users
    let cleaned = clean_dependency_name(dep);
    cleaned.ends_with(".so") || 
    cleaned.contains(".so=") ||
    cleaned.contains("lib") && cleaned.ends_with(".so")
}

fn extract_optional_dependencies(output: &str) -> Vec<String> {
    let mut in_optional_deps = false;
    let mut deps = Vec::new();
    
    for line in output.lines() {
        if line.starts_with("Optional Deps") {
            in_optional_deps = true;
            let deps_str = line.split(':').nth(1).unwrap_or("").trim();
            if deps_str != "None" && !deps_str.is_empty() {
                deps.push(deps_str.split(':').next().unwrap_or(deps_str).trim().to_string());
            }
            continue;
        }
        
        if in_optional_deps {
            if line.starts_with(' ') || line.starts_with('\t') {
                let dep = line.trim().split(':').next().unwrap_or("").trim();
                if !dep.is_empty() {
                    deps.push(dep.to_string());
                }
            } else {
                break;
            }
        }
    }
    
    deps
}

fn extract_field(output: &str, field: &str) -> String {
    for line in output.lines() {
        if line.starts_with(field) {
            return line.split(':').nth(1).unwrap_or("").trim().to_string();
        }
    }
    "Unknown".to_string()
}

fn get_required_by(package: &str) -> Result<Vec<String>, String> {
    let output = Command::new("pacman")
        .args(&["-Qi", package])
        .output()
        .map_err(|e| format!("Failed to get required by: {}", e))?;
    
    if !output.status.success() {
        return Ok(Vec::new());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Required By") {
            let req_str = line.split(':').nth(1).unwrap_or("").trim();
            if req_str == "None" {
                return Ok(Vec::new());
            }
            return Ok(req_str
                .split_whitespace()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect());
        }
    }
    
    Ok(Vec::new())
}

fn get_package_info_for_deps(package: &str) -> Result<(Vec<String>, Vec<String>, String, String, bool), String> {
    // println!("Getting package info for: {}", package); // Commented out to reduce spam
    
    // Try installed packages first
    let output = Command::new("pacman")
        .args(&["-Qi", package])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            // println!("Found {} in installed packages", package); // Commented out to reduce spam
            let stdout = String::from_utf8_lossy(&output.stdout);
            let deps = extract_dependencies(&stdout);
            let opt_deps = extract_optional_dependencies(&stdout);
            let version = extract_field(&stdout, "Version");
            let repo = extract_field(&stdout, "Repository");
            return Ok((deps, opt_deps, version, repo, true));
        }
    }
    
    // Try repository packages
    let output = Command::new("pacman")
        .args(&["-Si", package])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            println!("Found {} in repositories", package);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let deps = extract_dependencies(&stdout);
            let opt_deps = extract_optional_dependencies(&stdout);
            let version = extract_field(&stdout, "Version");
            let repo = extract_field(&stdout, "Repository");
            return Ok((deps, opt_deps, version, repo, false));
        } else {
            println!("pacman -Si failed for {}: {}", package, String::from_utf8_lossy(&output.stderr));
        }
    } else {
        println!("Failed to execute pacman -Si for {}", package);
    }
    
    // Try AUR packages
    let aur_helpers = ["yay", "paru"];
    for helper in &aur_helpers {
        println!("Trying AUR helper: {} for package: {}", helper, package);
        let output = Command::new(helper)
            .args(&["-Si", package])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                println!("Found {} in AUR via {}", package, helper);
                let stdout = String::from_utf8_lossy(&output.stdout);
                let deps = extract_dependencies(&stdout);
                let opt_deps = extract_optional_dependencies(&stdout);
                let version = extract_field(&stdout, "Version");
                return Ok((deps, opt_deps, version, "aur".to_string(), false));
            } else {
                println!("{} -Si failed for {}: {}", helper, package, String::from_utf8_lossy(&output.stderr));
            }
        } else {
            println!("Failed to execute {} -Si for {}", helper, package);
        }
    }
    
    Err(format!("Package {} not found in any repository", package))
}

fn clean_dependency_name(dep: &str) -> String {
    // Remove version constraints like >=, <=, =, >, <, etc.
    // Handle both pacman format (package>=1.0) and AUR format (package>1.0)
    let cleaned = dep.split(&['>', '<', '='][..])
        .next()
        .unwrap_or(dep)
        .trim();
    
    // Also handle cases where there might be spaces before version constraints
    let cleaned = cleaned.split_whitespace().next().unwrap_or(cleaned);
    
    // println!("Cleaned dependency: '{}' -> '{}'", dep, cleaned); // Commented out to reduce spam
    cleaned.to_string()
}
