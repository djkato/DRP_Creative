use std::fs;

#[derive(Debug)]
pub struct Config {
    pub exclude_keywords_list: Vec<String>,
    pub should_exclude_be_invisible: bool,
    pub portfolio_link: String,
    pub hide_portfolio_row: bool,
}

impl Config {
    pub fn load() -> Config {
        let default_config = use_default_config();

        //if no config file, parse default config file and try save it, then use it
        let config_file: Config = match fs::read_to_string(".drp_config") {
            Result::Err(_err) => {
                let def_config = parse_config_file(&default_config);
                make_config_file(&def_config);
                def_config
            }
            Result::Ok(val) => parse_config_file(&val),
        };
        config_file
    }
}
impl Config {
    pub fn exclude_project(&mut self, project_name: &String) {
        if self.exclude_keywords_list.contains(&project_name) {
            return;
        }
        self.exclude_keywords_list.push(project_name.clone());
        make_config_file(&self);
    }
}
fn parse_config_file(config_file: &String) -> Config {
    let mut exclude_keywords_list: Vec<String> = Vec::new();
    let mut should_exclude_be_invisible = false;
    let mut portfolio_link = String::new();
    let mut hide_portfolio_row = false;
    for (i, line) in config_file.lines().enumerate() {
        if line.contains("SHOULD_EXCLUDE_BE_ANONYMOUS:") {
            match config_file
                .lines()
                .nth(i + 1)
                .expect("index out of bounds")
                .to_lowercase()
                .as_str()
                .trim()
            {
                "n" => should_exclude_be_invisible = false,
                "no" => should_exclude_be_invisible = false,
                "y" => should_exclude_be_invisible = true,
                "yes" => should_exclude_be_invisible = true,
                _ => should_exclude_be_invisible = false,
            }
        }
        if line.contains("EXCLUDE_CHARACTERS_LIST:") {
            for arg_line in config_file.lines().skip(i + 1) {
                if arg_line.trim().contains("}") {
                    break;
                }
                exclude_keywords_list.push(arg_line.to_string())
            }
        }
        if line.contains("PORTFOLIO_LINK:") {
            for arg_line in config_file.lines().skip(i) {
                if arg_line.trim().contains("}") {
                    break;
                }
                portfolio_link = arg_line.to_string();
            }
        }
        if line.contains("HIDE_PORTFOLIO_ROW:") {
            match config_file
                .lines()
                .nth(i + 1)
                .expect("index out of bounds")
                .to_lowercase()
                .as_str()
                .trim()
            {
                "n" => hide_portfolio_row = false,
                "no" => hide_portfolio_row = false,
                "y" => hide_portfolio_row = true,
                "yes" => hide_portfolio_row = true,
                _ => hide_portfolio_row = false,
            }
        }
    }
    Config {
        exclude_keywords_list,
        should_exclude_be_invisible,
        portfolio_link,
        hide_portfolio_row,
    }
}

fn make_config_file(config: &Config) {
    let mut config_file = String::from("SHOULD_EXCLUDE_BE_ANONYMOUS:{");
    config_file = config_file
        + match config.should_exclude_be_invisible {
            true => "\ny\n}\n",
            false => "\nn\n}\n",
        };

    config_file = config_file + "\nPORTFOLIO_LINK:{\ndjkato.net\n}\n";

    config_file = config_file + "\nEXCLUDE_CHARACTERS_LIST:{";
    for exclusive_word in &config.exclude_keywords_list {
        config_file = config_file + format!("\n{}", exclusive_word).as_str();
    }
    config_file = config_file + "\n}\n";

    match fs::write(".drp_config", config_file) {
        _ => (),
    }
}

fn use_default_config() -> String {
    let str = String::from(
        "SHOULD_EXCLUDE_BE_ANONYMOUS:{
n
}

PORTFOLIO_LINK:{
djkato.net
}

EXCLUDE_CHARACTERS_LIST:{
no_drp
}
",
    );
    str
}
