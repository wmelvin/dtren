use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to create a temporary directory and file for testing
fn setup_test_file(file_name: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join(file_name);
    fs::write(&file_path, "test content").expect("Failed to create test file");
    (temp_dir, file_path)
}

#[test]
fn test_rename_normal_file() {
    let (_temp_dir, file_path) = setup_test_file("test.txt");
    
    // Run dtren on the file
    let output = Command::new(env!("CARGO_BIN_EXE_dtren"))
        .arg(file_path.to_str().unwrap())
        .output()
        .expect("Failed to execute dtren");

    // Should succeed with exit code 0
    assert_eq!(output.status.code(), Some(0), 
        "Expected success but got: stderr={}", 
        String::from_utf8_lossy(&output.stderr));

    // Original file should no longer exist
    assert!(!file_path.exists(), "Original file should not exist after rename");

    // Check that a renamed file exists with the expected pattern
    let parent_dir = file_path.parent().unwrap();
    let entries: Vec<_> = fs::read_dir(parent_dir)
        .expect("Failed to read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Should have exactly one file matching pattern: test-YYYYMMDD_HHMMSS.txt
    let renamed_files: Vec<_> = entries.iter()
        .filter(|name| name.starts_with("test-") && name.ends_with(".txt"))
        .collect();
    
    assert_eq!(renamed_files.len(), 1, 
        "Expected exactly one renamed file, found: {:?}", entries);

    let renamed_name = renamed_files[0];
    
    // Verify the timestamp format in the filename (YYYYMMDD_HHMMSS)
    // Expected format: test-YYYYMMDD_HHMMSS.txt
    let expected_len = "test-".len() + 15 + ".txt".len(); // 15 is YYYYMMDD_HHMMSS
    assert_eq!(renamed_name.len(), expected_len,
        "Renamed file name has unexpected length: {}", renamed_name);
    
    // Extract the timestamp portion
    let timestamp_part = &renamed_name["test-".len()..renamed_name.len() - ".txt".len()];
    assert_eq!(timestamp_part.len(), 15, // YYYYMMDD_HHMMSS is 15 characters
        "Timestamp should be 15 characters, got: {}", timestamp_part);
    
    // Verify format: 8 digits, underscore, 6 digits
    assert!(timestamp_part.chars().nth(8) == Some('_'),
        "Expected underscore at position 8 in timestamp: {}", timestamp_part);
    
    let date_part = &timestamp_part[..8];
    let time_part = &timestamp_part[9..];
    assert!(date_part.chars().all(|c| c.is_ascii_digit()),
        "Date part should be all digits: {}", date_part);
    assert!(time_part.chars().all(|c| c.is_ascii_digit()),
        "Time part should be all digits: {}", time_part);
}

#[test]
fn test_rename_tarball_file() {
    let (_temp_dir, file_path) = setup_test_file("test.tar.gz");
    
    // Run dtren on the tarball file
    let output = Command::new(env!("CARGO_BIN_EXE_dtren"))
        .arg(file_path.to_str().unwrap())
        .output()
        .expect("Failed to execute dtren");

    // Should succeed with exit code 0
    assert_eq!(output.status.code(), Some(0),
        "Expected success but got: stderr={}",
        String::from_utf8_lossy(&output.stderr));

    // Original file should no longer exist
    assert!(!file_path.exists(), "Original file should not exist after rename");

    // Check that a renamed file exists with the expected pattern
    let parent_dir = file_path.parent().unwrap();
    let entries: Vec<_> = fs::read_dir(parent_dir)
        .expect("Failed to read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Should have exactly one file matching pattern: test-YYYYMMDD_HHMMSS.tar.gz
    let renamed_files: Vec<_> = entries.iter()
        .filter(|name| name.starts_with("test-") && name.ends_with(".tar.gz"))
        .collect();
    
    assert_eq!(renamed_files.len(), 1,
        "Expected exactly one renamed file, found: {:?}", entries);

    let renamed_name = renamed_files[0];
    
    // Verify the format: test-YYYYMMDD_HHMMSS.tar.gz
    // The timestamp should be inserted before .tar.gz
    assert!(renamed_name.starts_with("test-"),
        "Renamed file should start with 'test-': {}", renamed_name);
    assert!(renamed_name.ends_with(".tar.gz"),
        "Renamed file should end with '.tar.gz': {}", renamed_name);
    
    // Extract the timestamp portion (between "test-" and ".tar.gz")
    let timestamp_part = &renamed_name["test-".len()..renamed_name.len() - ".tar.gz".len()];
    assert_eq!(timestamp_part.len(), 15, // YYYYMMDD_HHMMSS is 15 characters
        "Timestamp should be 15 characters, got: {}", timestamp_part);
    
    // Verify format: 8 digits, underscore, 6 digits
    assert!(timestamp_part.chars().nth(8) == Some('_'),
        "Expected underscore at position 8 in timestamp: {}", timestamp_part);
}

#[test]
fn test_rename_file_no_extension() {
    let (_temp_dir, file_path) = setup_test_file("README");
    
    // Run dtren on the file without extension
    let output = Command::new(env!("CARGO_BIN_EXE_dtren"))
        .arg(file_path.to_str().unwrap())
        .output()
        .expect("Failed to execute dtren");

    // Should succeed with exit code 0
    assert_eq!(output.status.code(), Some(0),
        "Expected success but got: stderr={}",
        String::from_utf8_lossy(&output.stderr));

    // Original file should no longer exist
    assert!(!file_path.exists(), "Original file should not exist after rename");

    // Check that a renamed file exists with the expected pattern
    let parent_dir = file_path.parent().unwrap();
    let entries: Vec<_> = fs::read_dir(parent_dir)
        .expect("Failed to read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Should have exactly one file matching pattern: README-YYYYMMDD_HHMMSS
    let renamed_files: Vec<_> = entries.iter()
        .filter(|name| name.starts_with("README-"))
        .collect();
    
    assert_eq!(renamed_files.len(), 1,
        "Expected exactly one renamed file, found: {:?}", entries);

    let renamed_name = renamed_files[0];
    
    // Verify no extension is added
    assert!(!renamed_name.contains('.'),
        "File without extension should not gain one: {}", renamed_name);
    
    // Extract the timestamp portion
    let timestamp_part = &renamed_name["README-".len()..];
    assert_eq!(timestamp_part.len(), 15, // YYYYMMDD_HHMMSS is 15 characters
        "Timestamp should be 15 characters, got: {}", timestamp_part);
}

#[test]
fn test_rename_tar_bz2_file() {
    let (_temp_dir, file_path) = setup_test_file("archive.tar.bz2");
    
    // Run dtren on the tar.bz2 file
    let output = Command::new(env!("CARGO_BIN_EXE_dtren"))
        .arg(file_path.to_str().unwrap())
        .output()
        .expect("Failed to execute dtren");

    // Should succeed with exit code 0
    assert_eq!(output.status.code(), Some(0),
        "Expected success but got: stderr={}",
        String::from_utf8_lossy(&output.stderr));

    // Check that the renamed file has the correct format: archive-YYYYMMDD_HHMMSS.tar.bz2
    let parent_dir = file_path.parent().unwrap();
    let entries: Vec<_> = fs::read_dir(parent_dir)
        .expect("Failed to read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let renamed_files: Vec<_> = entries.iter()
        .filter(|name| name.starts_with("archive-") && name.ends_with(".tar.bz2"))
        .collect();
    
    assert_eq!(renamed_files.len(), 1,
        "Expected exactly one renamed file with .tar.bz2 extension, found: {:?}", entries);
}
