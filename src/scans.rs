use std::fs;
use std::{error::Error, fmt::Display, fs::DirEntry, path::PathBuf};

use crate::actions::NoteDate;
use crate::settings::Paths;

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

#[derive(Debug, Clone)]
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

    pub fn to_path(&self, paths: &Paths) -> PathBuf {
        let path = paths.scans_path.clone() + "/" + &self.id;
        PathBuf::from(path)
    }

    pub fn populate_scans(settings: &Paths) -> Vec<Scan> {
        // Read all files in scan directory
        let dirs = std::fs::read_dir(settings.scans_path.clone()).unwrap();

        // lets sort the paths
        let mut dir_vec: Vec<DirEntry> = dirs.map(|p| p.unwrap()).collect();
        dir_vec.sort_by_key(|dir| dir.path());

        // Now only populate directories
        // TODO: Use filter instead of for loop
        let mut scans = vec![];
        for path in dir_vec {
            let path = path.path();
            if path.is_dir() {
                let scan = Scan::from_path(&path);
                scans.push(scan);
            }
        }

        scans
    }

    pub fn delete(&self, paths: &Paths) -> Result<(), DeleteError> {
        // Delete the directory
        let path = self.to_path(paths);
        std::fs::remove_dir_all(&path).map_err(|_| DeleteError)
    }

    pub fn post(&self, date: &NoteDate, paths: &Paths) -> Result<(), PostError> {
        let note_path = date.to_path();
        let note_path = PathBuf::from(paths.notes_path.clone()).join(note_path);
        std::fs::create_dir_all(&note_path).map_err(|_| PostError)?;

        let note_number = find_next_note_number(note_path.clone());
        println!("note_number: {}", note_number);
        let note_path = note_path.join(note_number);
        println!("note_path: {:?}", note_path);
        std::fs::create_dir_all(&note_path).map_err(|_| PostError)?;

        // Copy image to note path
        let source = self.to_path(paths).join("result.jpg");
        let target = note_path.join("result.jpg");
        println!("source: {:?}", source);

        // move source to target. Coping and then deleting as source and target are on different
        // filesystems
        fs::copy(source, target).map_err(|_| PostError)?;

        // Delete the scan directory
        self.delete(paths).map_err(|_| PostError)
    }
}
