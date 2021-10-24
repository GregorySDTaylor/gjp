use std::process::Command;
use ureq;
use std::fs;
use serde_json::Value;
use std::env;


fn main() {
    // https://developers.google.com/drive/api/v3/reference/files/export
    // https://developers.google.com/oauthplayground
    // https://console.cloud.google.com/home/dashboard?
    let args: Vec<String> = env::args().collect();
    println!("getting document metadata...");
    let get_path = format!("https://www.googleapis.com/drive/v3/files/{}", args[1]);
    let auth_header = format!("Bearer {}", args[2]);
    let get_response = ureq::get(&get_path)
        .set("Authorization", &auth_header)
        .call()
        .expect("failed to get document metadata")
        .into_string()
        .expect("failed to stringify document metadata");
    let get_value: Value = serde_json::from_str(&get_response)
        .expect("failed to deserialize document metadata");
    println!("getting java source...");
    let export_path = format!("https://www.googleapis.com/drive/v3/files/{}/export?mimeType=text/plain", args[1]);
    let export_response = ureq::get(&export_path)
        .set("Authorization", &auth_header)
        .call()
        .expect("failed to get java source")
        .into_string()
        .expect("failed to stringify java source");
    let name_with_extension = get_value["name"].as_str()
        .expect("failed to get file name");
    let name = &name_with_extension[..name_with_extension.len()-5];
    let source_path = format!("java_src/{}.java", &name);
    fs::create_dir_all("java_src")
        .expect("failed to create java source folder");
    fs::write(&source_path, &export_response[3..])
        .expect("Unable to write file");
    println!("compiling java source...");
    fs::create_dir_all("compiled_java")
        .expect("failed to create compiled java folder");
    let mut javac = Command::new("javac")
        .arg(&source_path)
        .arg("-d")
        .arg("compiled_java")
        .spawn()
        .expect("java source failed to compile");
    let exit_code = javac.wait()
        .expect("failed to wait on javac");
    println!("javac completed with {}", exit_code);
    println!("running java bytecode...");
    let mut java = Command::new("java")
        .arg("-classpath")
        .arg("compiled_java")
        .arg(&name)
        .spawn()
        .expect("java bytecode failed to run");
    let exit_code = java.wait()
        .expect("failed to wait on java");
    println!("java completed with {}", exit_code);
}
