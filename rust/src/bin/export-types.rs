use ts_rs::TS;
use london_classical::core::Concert;

fn main() {
    let output_dir = env!("CARGO_MANIFEST_DIR").to_string() + "/../src/lib/bindings";
    match Concert::export_all_to(&output_dir) {
        Ok(_) => println!("Exported types to {}", output_dir),
        Err(e) => eprintln!("Error: {}", e),
    }
}
