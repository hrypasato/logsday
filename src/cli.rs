use clap::Parser;
use crate::model::Level;

#[derive(Parser)]
#[command(name = "tareas", about = "Registra tareas con timesptamp")]
pub struct Cli {
    pub task: String,

    #[arg(short, long, value_enum, default_value = "media")]
    pub level:Level,

    #[arg(short, long, default_value = "Sistemas")]
    pub source:String,
}