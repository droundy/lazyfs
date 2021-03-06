//! A crate for lazy error-ignoring versions of `fs` functions.
//!
//! # Examples
//!
//! The following example illustrates reading a directory and printing
//! the entries.  If the directory does not exist or is not readable,
//! it will not print anything.
//!
//! ```
//! extern crate lazyfs;
//! for de in lazyfs::read_dir("path/to/directory") {
//!     println!("Got entry {:?}", de.path());
//! }
//! ```

/// The iterator over entries in a directory.  It returns all the
/// entries that can be read without error.
pub enum ReadDir {
    Ok(std::fs::ReadDir),
    Err,
}
impl Iterator for ReadDir {
    type Item = std::fs::DirEntry;
    fn next(&mut self) -> Option<std::fs::DirEntry> {
        match *self {
            ReadDir::Err => None,
            ReadDir::Ok(ref mut rd) => {
                loop {
                    match rd.next() {
                        Some(Ok(de)) => { return Some(de); }
                        Some(Err(_)) => {}
                        None => { return None; }
                    }
                }
            },
        }
    }
}

/// Read the contents of a directory.  In case of error, don't return
/// any contents.
pub fn read_dir<P: AsRef<std::path::Path>>(path: P) -> ReadDir {
    match std::fs::read_dir(path) {
        Ok(rd) => ReadDir::Ok(rd),
        _ => ReadDir::Err,
    }
}

#[test]
fn read_dotgit() {
    let git_stuff: Vec<_> = read_dir(".git").map(|p| {
        p.path().to_string_lossy().to_string()
    }).collect();
    println!("{:?}", git_stuff);
    assert!(git_stuff.contains(&".git/refs".to_string()));
    assert!(git_stuff.contains(&".git/config".to_string()));
    assert!(git_stuff.contains(&".git/info".to_string()));
    assert!(git_stuff.contains(&".git/objects".to_string()));
    assert!(git_stuff.contains(&".git/index".to_string()));
    assert!(git_stuff.contains(&".git/hooks".to_string()));
    assert!(git_stuff.contains(&".git/description".to_string()));
    assert!(git_stuff.contains(&".git/HEAD".to_string()));
}

#[test]
fn read_src() {
    let stuff: Vec<_> = read_dir("src").filter(|p| {
        p.path().extension() == Some(&std::ffi::OsStr::new("rs"))
    }).map(|p| {
        p.path().to_string_lossy().to_string()
    }).collect();
    println!("{:?}", stuff);
    assert_eq!(stuff.len(), 1);
}

#[test]
fn read_nonexistent() {
    let stuff: Vec<_> = read_dir("this does not exist").collect();
    assert_eq!(stuff.len(), 0);
}
