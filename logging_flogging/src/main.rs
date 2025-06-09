use logging_flogging::write_sample_log;

fn main() {
    match write_sample_log(&"sample_string_literals".to_string()) {
        Ok(_) => println!("data written successfully"),
        Err(e) => println!("failed to write data with: {}", e),
    }
}
