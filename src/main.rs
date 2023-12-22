use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn escape_special_characters(regex_string: &str) -> String {
    let special_characters = r".+*?[^]$(){}";
    let regex = Regex::new(&format!("[{}]", regex::escape(special_characters))).unwrap();
    regex.replace_all(regex_string, |caps: &regex::Captures| format!("\\{}", &caps[0])).to_string()
}

fn rename_files(pattern: &str, prefix: &str) {
    let files = fs::read_dir(".").unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().ok().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    let filename_pattern: Vec<&str> = pattern.split('$').collect();
    let old_prefix = escape_special_characters(filename_pattern[0]);
    let old_postfix = escape_special_characters(filename_pattern[2]);

    for file in files {
        let re_str = format!(r"{}(\d+){}", old_prefix, old_postfix);
        let regex = Regex::new(&re_str).unwrap();

        if let Some(captures) = regex.captures(&file) {
            if let Some(number) = captures.get(1) {
                let file_ext = Path::new(&file).extension().and_then(|ext| ext.to_str()).unwrap_or("");
                let new_file_name = format!("{}{}.{}", prefix, number.as_str(), file_ext);
                fs::rename(&file, &new_file_name).expect("Error renaming file");
                println!("Renamed {} to {}", file, new_file_name);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <pattern> <prefix>", args[0]);
    } else {
        let pattern = &args[1];
        let prefix = &args[2];
        rename_files(pattern, prefix);
    }
}
