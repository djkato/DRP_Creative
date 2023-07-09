use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub keywords_list: Vec<String>,
    pub show_default_when_excluded: bool,
    pub portfolio_link: String,
    pub hide_portfolio_row: bool,
    pub should_list_include_or_exclude: IncludeExclude,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keywords_list: Vec::new(),
            should_list_include_or_exclude: IncludeExclude::Exclude,
            portfolio_link: "djkato.net".to_owned(),
            hide_portfolio_row: false,
            show_default_when_excluded: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IncludeExclude {
    Include,
    Exclude,
}

impl Config {
    pub fn add_project(&mut self, project_name: &String) {
        if self.keywords_list.contains(&project_name) {
            return;
        }
        self.keywords_list.push(project_name.clone());
        self.write();
    }

    pub fn remove_project(&mut self, project_name: &String) {
        if self.keywords_list.contains(&project_name) {
            self.keywords_list.remove(
                self.keywords_list
                    .iter()
                    .position(|proj| proj == project_name)
                    .unwrap(),
            );
            self.write();
        }
    }
    pub fn load_from_file(&mut self) {
        if !std::path::Path::new("drp_config.toml").exists() {
            self.write();
        } else {
            let mut file = fs::File::options()
                .read(true)
                .open("drp_config.toml")
                .expect(
                    "Config file exists but can't be acccessed. Check permissions for the file.",
                );

            let mut content = String::new();
            file.read_to_string(&mut content)
                .expect("config file not valid");

            if content.is_empty() {
                self.write();
            }

            *self = toml::from_str::<Config>(content.as_str())
                .expect("Config file exists, but the format of it is wrong.");
        }
    }

    pub fn write(&self) {
        let mut file = fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("drp_config.toml")
            .expect("Couldn't write to file");
        let buff =
            toml::ser::to_string_pretty(self).expect("Something is wrong with the default config");

        file.write_all(buff.as_bytes())
            .expect("Failed to write files");
    }
}
