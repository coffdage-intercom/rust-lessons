use std::fs::File;
use std::fs;

fn main() {
    // Files
      // use std::fs::File;
    File::create("test.txt").unwrap();
      // use std::fs;
    fs::remove_file("test.txt").unwrap;
    // Folders
      // use std::fs;
    fs::create_dir("TEST").unwrap;
      // use std::fs;
    fs::remove_dir("TEST").unwrap;
}