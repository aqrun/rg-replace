use crate::Cli;
use std::process::Command;
use std::path::PathBuf;
use crate::models::FileData;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use anyhow::{anyhow, Result};

pub fn default_run(cli: &Cli) {
    let files = find_files(cli).expect("Find file err");
    println!("cli: {:?}", cli);

    if cli.confirm {
        if let Err(err) = replace_file_content(&files) {
            println!("替换操作失败：{:?}", err);
        }
    } else {
        show_dry_run_contents(&files);
    }
}

fn show_dry_run_contents(files: &[FileData]) {
    println!("Dry Run data:");
    files.into_iter().for_each(|item| {
        println!("{}\n{}\n{}\n", item.path, item.source_str, item.replace_with);
    })
}

// -r "use oicnp_core::prelude::sea_orm_migration::prelude::*;"
fn find_files(cli: &Cli) -> Result<Vec<FileData>> {
    // "sea_orm_migration::prelude::\\*;";
    let search = String::from(&cli.pattern);
    let search_path = PathBuf::from(&cli.search_path);

    let rg_cmd = format!("rg -0 -s -e \"{}\" {}", search, search_path.to_str().unwrap());

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &rg_cmd])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&rg_cmd)
            .output()?
    };

    let stdout = String::from_utf8(output.stdout)?;
    let arr: Vec<&str> = stdout.split("\n").collect();
    // println!("111111 {:?}", arr);
    let valid_str_arr: Vec<&str> = arr.into_iter().filter(|item| {
        let target = *item;
        return !target.eq("");
    }).collect();

    let data: Vec<FileData> = valid_str_arr.into_iter().map(|item| {
        let item_arr: Vec<&str> = item.split("\0").collect();
        // println!("{:?}", item_arr);
        let file = *item_arr.get(0).unwrap();
        let content = *item_arr.get(1).unwrap();

        let file_data = FileData {
            path: String::from(file),
            source_str: String::from(content),
            replace_with: String::from(&cli.replace_with),
        };
        return file_data;
    }).collect();

    Ok(data)
    // println!("cmd {}", &rg_cmd);
    // println!("output {:?}", data);
}

fn replace_file_content(files: &[FileData]) -> Result<String> {
    for item in files.into_iter() {
        let content = match read_file_content(&item.path.as_str()) {
            Ok(content) => content,
            Err(err) => {
                return Err(anyhow!("Read file Err, {:?}", err));
            }
        };
        let new_content = content.replace(&item.source_str, &item.replace_with);

        match write_file_content(&item.path.as_str(), &new_content) {
            Err(err) => {
                return Err(anyhow!("Write file Err, {:?}", err));
            },
            _ => {

            }
        }
        println!("文件操作完成：{}", &item.path.as_str());
    }

    Ok(format!("Replace success"))
}

fn read_file_content(file_path: &str) -> Result<String> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file_content(file_path: &str, content: &str) -> Result<String> {
    fs::write(file_path, content.as_bytes())?;
    Ok(format!("File Write success"))
}

