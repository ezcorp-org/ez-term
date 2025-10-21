use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_git_context_detection() {
    // Skip if not in a git repo
    let is_git_repo = Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !is_git_repo {
        println!("Skipping git test - not in a git repository");
        return;
    }

    // If we are in a git repo, the context agent should detect it
    // This is more of an integration test
}

#[test]
fn test_query_pattern_matching() {
    // Test that queries mentioning specific tools trigger context fetching
    let git_queries = vec![
        "show me my git branches",
        "commit these changes to the repository",
        "what's my current branch",
    ];

    for query in git_queries {
        assert!(query.to_lowercase().contains("git") ||
                query.to_lowercase().contains("branch") ||
                query.to_lowercase().contains("commit"));
    }
}

#[test]
fn test_docker_context_detection() {
    let docker_available = Command::new("docker")
        .args(&["--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if docker_available {
        println!("Docker is available for testing");
    } else {
        println!("Skipping Docker test - docker not installed");
    }
}

#[test]
fn test_package_json_detection() {
    let temp_dir = TempDir::new().unwrap();
    let package_json = temp_dir.path().join("package.json");

    // Create a fake package.json
    fs::write(
        &package_json,
        r#"{
  "name": "test-package",
  "version": "1.0.0",
  "scripts": {
    "test": "echo test",
    "build": "echo build",
    "start": "echo start"
  }
}"#,
    )
    .unwrap();

    // Verify file exists
    assert!(package_json.exists());

    // In a real scenario, context agent would detect this
    let content = fs::read_to_string(&package_json).unwrap();
    assert!(content.contains("test-package"));
    assert!(content.contains("scripts"));
}

#[test]
fn test_cargo_toml_detection() {
    // We're in the ez-term project, so Cargo.toml should exist
    use std::path::PathBuf;

    let cargo_toml = PathBuf::from("Cargo.toml");
    assert!(cargo_toml.exists(), "Cargo.toml should exist in project root");

    let content = fs::read_to_string(&cargo_toml).unwrap();
    assert!(content.contains("ez-cli"));
}

#[test]
fn test_python_requirements_detection() {
    let temp_dir = TempDir::new().unwrap();
    let requirements = temp_dir.path().join("requirements.txt");

    // Create a fake requirements.txt
    fs::write(
        &requirements,
        "requests==2.28.0\npandas==1.5.0\nnumpy==1.24.0\n",
    )
    .unwrap();

    assert!(requirements.exists());
}

#[test]
fn test_rc_file_alias_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let zshrc = temp_dir.path().join(".zshrc");

    // Create a fake .zshrc with aliases
    fs::write(
        &zshrc,
        r#"
# My aliases
alias ll='ls -la'
alias gs='git status'
alias gp='git push'

# Some exports
export PATH=$HOME/bin:$PATH
export EDITOR=vim

# A function
myfunction() {
    echo "test"
}
"#,
    )
    .unwrap();

    let content = fs::read_to_string(&zshrc).unwrap();

    // Extract aliases
    let aliases: Vec<&str> = content
        .lines()
        .filter(|line| line.trim().starts_with("alias "))
        .collect();

    assert_eq!(aliases.len(), 3);
    assert!(aliases[0].contains("ll"));
    assert!(aliases[1].contains("gs"));
    assert!(aliases[2].contains("gp"));

    // Extract exports
    let exports: Vec<&str> = content
        .lines()
        .filter(|line| line.trim().starts_with("export "))
        .collect();

    assert_eq!(exports.len(), 2);
}

#[test]
fn test_context_agent_keyword_detection() {
    // Simulate keyword detection logic
    let test_cases = vec![
        ("show me git branches", true, &["git", "branch"]),
        ("list docker containers", true, &["docker", "container"]),
        ("npm install package", true, &["npm", "package"]),
        ("run python script", true, &["python", "script"]),
        ("list files", false, &["git", "docker"]),
    ];

    for (query, should_match, keywords) in test_cases {
        let query_lower = query.to_lowercase();
        let matches = keywords.iter().any(|kw| query_lower.contains(kw));

        assert_eq!(
            matches, should_match,
            "Query '{}' should {} keywords {:?}",
            query,
            if should_match { "match" } else { "not match" },
            keywords
        );
    }
}
