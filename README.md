# logsday

CLI tool to save tasks done with timestamp

#### How to use
```bash
tareas "task name" -l difficult -s source
```

#### Build
```bash
cargo build --release          # binario en target/release/tareas
cargo install --path .         # lo copia a ~/.cargo/bin
logsday "conectar proyector" -l baja -s uga
```

#### Todo

- Aceptar fecha y hora como argumento