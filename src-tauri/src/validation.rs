use regex::Regex;
use std::collections::HashSet;

pub fn validate_package_name(package: &str) -> Result<(), String> {
    if package.is_empty() {
        return Err("Package name cannot be empty".to_string());
    }

    if package.len() > 255 {
        return Err("Package name too long (max 255 characters)".to_string());
    }

    let package_regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9@+._-]*$")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    if !package_regex.is_match(package) {
        return Err(format!(
            "Invalid package name '{}'. Package names can only contain letters, numbers, @, +, ., _, - and must start with a letter or number",
            package
        ));
    }

    let dangerous_patterns = [
        "..", "//", "\\", "|", "&", ";", "`", "$", "(", ")", "{", "}", "[", "]",
        "<", ">", "?", "*", "!", "~", "#", "%", "^", "=", " ", "\t", "\n", "\r"
    ];

    for pattern in &dangerous_patterns {
        if package.contains(pattern) {
            return Err(format!(
                "Package name '{}' contains dangerous character(s): '{}'",
                package, pattern
            ));
        }
    }

    Ok(())
}

pub fn validate_repository_name(repo: &str) -> Result<(), String> {
    if repo.is_empty() {
        return Err("Repository name cannot be empty".to_string());
    }

    let valid_repos: HashSet<&str> = [
        "core", "extra", "community", "multilib", "testing", "community-testing",
        "multilib-testing", "staging", "aur", "local"
    ].iter().cloned().collect();

    if !valid_repos.contains(repo) {
        let repo_regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9._-]*$")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        if !repo_regex.is_match(repo) {
            return Err(format!(
                "Invalid repository name '{}'. Repository names can only contain letters, numbers, ., _, - and must start with a letter or number",
                repo
            ));
        }
    }

    Ok(())
}

pub fn validate_aur_helper(helper: &str) -> Result<(), String> {
    let valid_helpers = ["yay", "paru"];

    if !valid_helpers.contains(&helper) {
        return Err(format!(
            "Invalid AUR helper '{}'. Supported helpers: {}",
            helper,
            valid_helpers.join(", ")
        ));
    }

    Ok(())
}

pub fn validate_file_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("File path cannot be empty".to_string());
    }

    if path.contains("..") || path.contains("//") {
        return Err("Path traversal detected in file path".to_string());
    }

    let dangerous_chars = ['|', '&', ';', '`', '$', '(', ')', '{', '}', '[', ']', '<', '>', '?', '*', '!', '~', '#', '%', '^', '='];

    for ch in &dangerous_chars {
        if path.contains(*ch) {
            return Err(format!("Dangerous character '{}' detected in file path", ch));
        }
    }

    Ok(())
}

pub fn sanitize_package_name(package: &str) -> String {
    package
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '@' | '+' | '.' | '_' | '-'))
        .collect()
}

pub fn sanitize_repository_name(repo: &str) -> String {
    repo
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-'))
        .collect()
}
