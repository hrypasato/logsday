use std::error::Error;
use std::fs;
use std::path::PathBuf;
use crate::model::Entry;

// Linux: ~/.local/share/tareas/tasks.json
fn data_path() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::data_dir()
        .ok_or("no se pudo encontrar el directorio de datos")?
        .join("tareas");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("tasks.json"))
}

pub fn save(entry: Entry) -> Result<(), Box<dyn Error>> {
    let path = data_path()?;

    let mut entries: Vec<Entry> = if path.exists() {
        serde_json::from_str(&fs::read_to_string(&path)?)?
    } else {
        Vec::new()
    };

    entries.push(entry);
    fs::write(&path, serde_json::to_string_pretty(&entries)?)?;
    println!("Guardado en {}", path.display());
    Ok(())
}