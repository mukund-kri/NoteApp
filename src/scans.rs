use std::{
    error::Error,
    fmt::Display,
    fs::DirEntry,
    path::{Path, PathBuf},
};

use crate::actions::NoteDate;

const SCAN_DIR: &str = "/home/mukund/computer/data/ClearScanner";
const NOTES_DIR: &str = "/home/mukund/notes";

#[derive(Debug, Clone)]
pub struct DeleteError;

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error deleting scan")
    }
}
impl Error for DeleteError {}

#[derive(Debug, Clone)]
pub struct PostError;

impl Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error posting scan")
    }
}

impl Error for PostError {}

#[derive(Debug, Clone, Default)]
pub struct Scan {
    pub id: String,
}

fn find_next_note_number(date_path: PathBuf) -> String {
    let mut note_number = 0;
    println!("date_path: {:?}", &date_path);
    let paths = std::fs::read_dir(&date_path).unwrap();

    // Sort alphabetically
    let mut path_vec: Vec<DirEntry> = paths.map(|p| p.unwrap()).collect();
    path_vec.sort_by_key(|dir| dir.path());

    // Pick the last note
    if path_vec.len() > 0 {
        let last_path = path_vec.last().unwrap().path();
        let last_name = last_path.file_name().unwrap().to_string_lossy();
        note_number = last_name.parse::<i32>().unwrap() + 1;
    }

    format!("{:03}", note_number)
}

impl Scan {
    pub fn from_path(path: &PathBuf) -> Self {
        let id = path.file_name().unwrap().to_string_lossy().to_string();
        Scan { id }
    }

    pub fn to_path(&self) -> PathBuf {
        let path = SCAN_DIR.to_string() + "/" + &self.id;
        PathBuf::from(path)
    }

    pub fn populate_scans() -> Vec<Scan> {
        // Exit app if scan directory does not exist
        if !PathBuf::from(SCAN_DIR).exists() {
            println!("Scan directory does not exist");
            std::process::exit(1);
        }

        // Read all files in scan directory
        let paths = std::fs::read_dir(SCAN_DIR).unwrap();

        // lets sort the paths
        let mut path_vec: Vec<DirEntry> = paths.map(|p| p.unwrap()).collect();
        path_vec.sort_by_key(|dir| dir.path());

        // Now only populate directories
        let mut scans = vec![];
        for path in path_vec {
            let path = path.path();
            if path.is_dir() {
                let scan = Scan::from_path(&path);
                scans.push(scan);
            }
        }

        scans
    }

    pub fn delete(&self) -> Result<(), DeleteError> {
        // Delete the directory
        let path = self.to_path();
        std::fs::remove_dir_all(&path).map_err(|_| DeleteError)
    }

    pub fn post(&self, date: &NoteDate) -> Result<(), PostError> {
        let note_path = date.to_path();
        let note_path = PathBuf::from(NOTES_DIR).join(note_path);
        std::fs::create_dir_all(&note_path).map_err(|_| PostError)?;

        let note_number = find_next_note_number(note_path.clone());
        println!("note_number: {}", note_number);
        let note_path = note_path.join(note_number);
        println!("note_path: {:?}", note_path);
        std::fs::create_dir_all(&note_path).map_err(|_| PostError)?;

        // Copy image to note path
        let source = self.to_path().join("result.jpg");
        let target = note_path.join("result.jpg");
        println!("source: {:?}", source);
        // move source to target
        std::fs::rename(source, target).map_err(|_| PostError)?;

        // Delete the scan directory
        self.delete().map_err(|_| PostError)
    }
}
