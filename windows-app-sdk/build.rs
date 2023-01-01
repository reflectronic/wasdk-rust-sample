use std::{fs, io::Write};

use thiserror::Error;
use windows_bindgen::{namespace, namespace_impl, Gen};
use windows_metadata::reader::{File, Reader};

#[derive(Debug, Error)]
pub enum GenError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Var(#[from] std::env::VarError),
    #[error("Missing Path")]
    MissingPath(std::path::PathBuf)
}

fn main() -> Result<(), GenError> {
    let mut winmd_path = std::path::PathBuf::from("");
    winmd_path.push("winmd");
    let winmd_files: Vec<_> = fs::read_dir(&winmd_path)
        .unwrap()
        .map(|path| {
            let path = path.expect("Couldn't open winmd");
            let os = path.path().into_os_string();
            let path_str = os.to_str().expect("Problem");
            File::new(path_str).expect(path_str)
        })
        .collect();
        
    let metadata_reader = Reader::new(&winmd_files);
    let tree = metadata_reader
        .tree("Microsoft.Windows.System.Power", &[])
        .map_or_else(|| Err(GenError::MissingPath(winmd_path)), Ok)?;

    let mut gen = Gen::new(&metadata_reader);
    gen.namespace = tree.namespace;

    let mut source_path = std::path::PathBuf::from("");
    source_path.push("src/Microsoft/Windows/System/Power/mod.rs");

    if let Some(p) = source_path.parent() { 
        fs::create_dir_all(p)? 
    }; 
    
    let mut source_file = fs::File::create(source_path.clone())?;

    source_file.write_all(namespace(&gen, &tree).as_bytes())?;
    source_file.write_all(namespace_impl(&gen, &tree).as_bytes())?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&source_path);
    cmd.output()?;
        
    Ok(())
}
