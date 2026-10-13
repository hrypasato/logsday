use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{Write};
use std::path::PathBuf;
use crate::model::Entry;

// Linux: ~/.local/share/tareas/tasks.json
fn data_path() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::data_local_dir()
        .ok_or("no se pudo encontrar el directorio de datos")?
        .join("tareas");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("tasks.jsonl"))
}

pub fn save(entry: &Entry) -> Result<(), Box<dyn Error>> {
    let path = data_path()?;

    let mut file = OpenOptions::new()
        .create(true) // si no existe, lo crea
        .append(true) // siempre escribe al final
        .open(&path)?;

    let line = serde_json::to_string(entry)?;
    writeln!(file, "{line}")?;

    println!("Guardado en {}", path.display());
    Ok(())
}