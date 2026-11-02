# Plan de aprendizaje de Rust multiplataforma

Plan práctico para Rust estable en 2026. Usa `rustup` para instalar y actualizar
la toolchain estable, y Cargo para compilar, probar y ejecutar. Los ejemplos de
este proyecto están en `src/`.

## Ruta

| Etapa | Caso de uso | Aprende | Ejercicio | Terminado cuando... |
|---|---|---|---|---|
| 1. Entorno y fundamentos | Una CLI que funcione en Windows, Linux y macOS | `rustup`, Cargo, tipos, funciones, control de flujo, módulos y `Result` | Ejecuta `cargo run -- help` y añade un comando | `cargo check` y `cargo run` funcionan sin errores |
| 2. Ownership y memoria | Leer y procesar un archivo sin copias innecesarias | ownership, borrowing, referencias, lifetimes, stack/heap, `String`, `&str`, `Copy`, `Clone`, `Drop` | Inspecciona `src/text_stats.rs` y añade una métrica que tome `&str` | Puedes explicar qué valor se mueve y qué valores se prestan |
| 3. Tipos y diseño | Representar resultados y errores de dominio | structs, enums, `Option`, `Result`, `match`, traits, generics e iteradores | Extiende el resumen de texto con una opción para filtrar palabras | El flujo distingue datos ausentes de errores |
| 4. Colecciones y datos | Analizar un archivo de texto | `Vec`, `HashMap`, archivos, JSON y persistencia | Añade una salida JSON y una prueba para ella | El análisis y la salida se prueban con datos pequeños |
| 5. Algoritmos y estructuras | Encontrar rutas en un mapa de nodos | Big O, recursividad, búsqueda y ordenamiento; listas, pilas, colas, árboles y grafos | Ejecuta BFS y DFS en `src/graph.rs`; después añade búsqueda binaria | Los recorridos visitan cada nodo alcanzable una sola vez |
| 6. Concurrencia y asincronía | Procesar trabajo independiente sin bloquear la aplicación | threads, canales, `Send`, `Sync`, `async`/`await` y runtimes | Compara `parallel` con `async-demo` | El resultado concurrente coincide con el secuencial y puedes explicar cuándo usar async |
| 7. Calidad y mantenimiento | Evitar regresiones al cambiar el programa | pruebas unitarias/de integración, documentación, `rustfmt`, Clippy, errores y límites de `unsafe` | Ejecuta `cargo test`, `cargo fmt --check` y `cargo clippy --all-targets` | Las pruebas pasan y Clippy no señala problemas nuevos |
| 8. Diseño y arquitectura | Mantener una CLI pequeña y extensible | módulos, traits, separación de responsabilidades y patrones cuando resuelvan un problema real | Separa un nuevo formato de salida detrás de una interfaz simple | Puedes añadir una salida sin duplicar el análisis |
| 9. Proyecto final | Una CLI de análisis de archivos | Integrar lectura, errores, algoritmos, pruebas y compilación multiplataforma | Añade argumentos, salida JSON y un workflow de CI para los tres sistemas | `cargo test` y la CLI funcionan en Windows, Linux y macOS |

## Ejecutar los ejemplos

```sh
cargo run -- help
cargo run -- stats README.md
cargo run -- graph
cargo run -- parallel 1000000 4
cargo run -- async-demo
cargo test
```

`stats <archivo>` muestra bytes, líneas, palabras y las palabras más frecuentes.
`graph` ejecuta BFS y DFS. `parallel <límite> <hilos>` suma los enteros desde
cero hasta el límite (exclusivo) usando threads. `async-demo` espera dos tareas
asíncronas concurrentemente con Tokio.
