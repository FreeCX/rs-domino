use crate::consts::GEN_HEADER;
use crate::error::BuildError;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;

// execute app and get stdout
fn execute(cmd: &str, args: &[&str]) -> Result<String, BuildError> {
    String::from_utf8(Command::new(cmd).args(args).output()?.stdout).map_err(|e| e.into())
}

// parse rustc and cargo stdout
fn parse(s: String) -> HashMap<String, String> {
    let mut res = HashMap::new();
    for line in s.lines() {
        let block: Vec<&str> = line.splitn(2, ':').collect();
        if block.len() == 1 {
            res.insert("header".to_string(), block[0].trim().to_string());
        } else {
            res.insert(block[0].to_string(), block[1].trim().to_string());
        }
    }
    res
}

// get packages info from Cargo.lock
fn app_packages() -> String {
    let mut counter = 0;
    let mut name = String::new();
    let mut packages = String::new();
    let mut name_flag = false;

    for line in include_str!("../Cargo.lock").lines() {
        if let Some(data) = line.strip_prefix("name = ") {
            name = data.to_string();
            name_flag = true;
        }
        if name_flag {
            if let Some(version) = line.strip_prefix("version = ") {
                name_flag = false;
                // (name, version)
                packages.push_str(&format!("    ({name}, {version}),\n"));
                counter += 1;
            }
        }
    }

    format!("pub static APP_PACKAGES: [(&str, &str); {counter}] = [\n{packages}];")
}

fn get_current_date() -> String {
    use chrono::prelude::*;
    let utc: DateTime<Utc> = Utc::now();
    utc.format("%Y-%m-%d %H:%M:%S %z").to_string()
}

pub fn generate_build_info() -> Result<(), BuildError> {
    let mut source_code = String::new();
    // file header
    source_code.push_str(GEN_HEADER);
    // add comment line
    source_code.push_str("// builder info\n");

    // rust and cargo info
    for (prefix, executable) in [("RUST", "rustc"), ("CARGO", "cargo")].iter() {
        let iterator = parse(execute(executable, &["-vV"])?);
        for (k, value) in iterator {
            let key = k.to_uppercase().replace("-", "_").replace(" ", "_");
            source_code.push_str(&format!("pub static {prefix}_{key}: &str = \"{value}\";\n"));
        }
    }

    // add project info
    let git_hash = include_str!("../.git/ORIG_HEAD").trim();
    let git_branch = include_str!("../.git/HEAD").rsplitn(2, '/').next().unwrap_or("-").trim();
    let current_build_date = get_current_date();
    let project_info = format!(
        "// project info\n\
        pub static GIT_PROJECT_BRANCH: &str = \"{git_branch}\";\n\
        pub static GIT_PROJECT_HASH: &str = \"{git_hash}\";\n\
        pub static PROJECT_BUILD_DATE: &str = \"{current_build_date}\"; // UTC+0\n\
        // packages\n"
    );
    source_code.push_str(&project_info);

    // add packages in Cargo.lock
    source_code.push_str(&app_packages());

    // and write to build.rs file
    File::create("src/build.rs").and_then(|mut file| write!(file, "{source_code}"))?;

    Ok(())
}
