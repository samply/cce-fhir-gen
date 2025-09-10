use std::fs;

use crate::models::cli::{Commands, OutputMode, ResourceType};
use crate::utils::DATA_FOLDER;

pub fn showcase_data(
    data: String,
    file_name: String,
    type_of_data: Commands,
    resource_type: ResourceType,
    output_mode: OutputMode,
) {
    match type_of_data {
        Commands::SyntheticData { .. } => {
            synthetic_data(data, file_name, resource_type, output_mode);
        }

        Commands::Catalogue { .. } => {
            catalogue(data, output_mode);
        }
    }
}

fn synthetic_data(
    data: String,
    file_name: String,
    resource_type: ResourceType,
    output_mode: OutputMode,
) {
    match output_mode {
        OutputMode::Screen => {
            println!("{}:", resource_type.as_str());
            println!("{data}");
            println!();
        }

        OutputMode::File => {
            let dir_path = format!("./{DATA_FOLDER}");
            if fs::exists(&dir_path).expect("dir exists error") {
                println!("{} already exists.", dir_path);
            } else {
                println!("creating {}.", &dir_path);
                fs::create_dir(&dir_path).expect("failed to create dir");
            }

            let with_extn = format!("{}.xml", file_name);
            let file_path = format!("{}/{}", &dir_path, with_extn);
            fs::write(file_path, data).expect("Unable to create XML file");
        }

        OutputMode::ApiCall => todo!(),
    }
}

fn catalogue(data: String, output_mode: OutputMode) {
    match output_mode {
        OutputMode::Screen => {
            println!("Catalogue JSON:");
            println!("{data}");
            println!();
        }

        OutputMode::File => {
            let dir_path = format!("./{DATA_FOLDER}");
            if fs::exists(&dir_path).expect("dir exists error") {
                println!("{} already exists.", dir_path);
            } else {
                println!("creating {}.", &dir_path);
                fs::create_dir(&dir_path).expect("failed to create dir");
            }

            let with_extn = format!("catalogue.json");
            let file_path = format!("{}/{}", &dir_path, with_extn);
            fs::write(file_path, data).expect("Unable to create the catalogue.json file");
        }

        OutputMode::ApiCall => todo!(),
    }
}
