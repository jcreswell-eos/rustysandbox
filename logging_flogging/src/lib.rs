use csv::Writer;
use std::fs::OpenOptions;

pub fn write_sample_log(file_name: &String) -> Result<(), String> {
    let root_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(val) => val,
        Err(e) => return Err(format!("failed to find manifest dir with error {:?}", e)),
    };
    let file_path = format!("{}/output/{}.csv", root_dir, file_name);
    if !std::path::Path::new(&file_path).exists() {
        match std::fs::File::create(&file_path) {
            Ok(_) => {}
            Err(e) => return Err(format!("Creating sample file failed with: {:?}", e)),
        }
    }
    let file = match OpenOptions::new().write(true).append(true).open(&file_path) {
        Ok(res) => res,
        Err(e) => {
            return Err(format!(
                "Unable to open file: {} because {:?}",
                &file_path, e
            ))
        }
    };

    let mut write = csv::Writer::from_writer(file);

    let string_one = String::from("testing");
    let string_two = String::from("stuff");

    let mut stringified_data: Vec<String> = vec![string_one, string_two];

    dbg!("stringified data says {:?}", &stringified_data);

    if write.write_record(stringified_data).is_ok() && write.flush().is_ok() {
        return Ok(());
    } else {
        return Err(format!("Unable to write data to file: {}", file_name));
    }
}
