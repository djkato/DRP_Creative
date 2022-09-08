#[derive(Debug)]
pub struct App {
    pub kind: AppKind,
    pub default_project_name: String,
    pub drp_client_id: String,
    pub process_search_string: String,
    pub config_search_string: String,
}
#[derive(Debug, PartialEq, Eq)]
pub enum AppKind {
    C4d,
    Maya,
    ThreeDsMax,
    AfterEffects,
    PremierePro,
    Illustrator,
    Photoshop,
    Indesign,
    DavinciResolve,
    Clarisse,
    Marvelous,
    Calvary,
    Ableton,
    FL,
}

pub struct Apps {
    C4d: App,
    Maya: App,
    ThreeDsMax: App,
    AfterEffects: App,
    PremierePro: App,
    Illustrator: App,
    Photoshop: App,
    Indesign: App,
    DavinciResolve: App,
    Clarisse: App,
    Marvelous: App,
    Calvary: App,
    Ableton: App,
    FL: App,
}
impl Apps {
    pub fn as_iter(&self) -> [&App; 14] {
        let APPS: [&App; 14] = [
            &self.C4d,
            &self.Maya,
            &self.ThreeDsMax,
            &self.AfterEffects,
            &self.PremierePro,
            &self.Illustrator,
            &self.Photoshop,
            &self.Indesign,
            &self.DavinciResolve,
            &self.Clarisse,
            &self.Marvelous,
            &self.Calvary,
            &self.Ableton,
            &self.FL,
        ];
        APPS
    }
}
impl AppKind {
    pub fn as_iter() -> [AppKind; 14] {
        let APPKINDS: [AppKind; 14] = [
            AppKind::C4d,
            AppKind::Maya,
            AppKind::ThreeDsMax,
            AppKind::AfterEffects,
            AppKind::PremierePro,
            AppKind::Illustrator,
            AppKind::Photoshop,
            AppKind::Indesign,
            AppKind::DavinciResolve,
            AppKind::Clarisse,
            AppKind::Marvelous,
            AppKind::Calvary,
            AppKind::Ableton,
            AppKind::FL,
        ];
        APPKINDS
    }
}
impl Apps {
    pub fn construct_apps() -> Apps {
        Apps {
            C4d: App {
                default_project_name: "Cinema 4D Project".to_string(),
                drp_client_id: "936296341250904065".to_string(),
                process_search_string: "Cinema 4D R".to_string(),
                config_search_string: "c4d".to_string(),
                kind: AppKind::C4d,
            },
            Maya: App {
                default_project_name: "Autodesk Maya Project".to_string(),
                drp_client_id: "1016369550276694106".to_string(),
                process_search_string: "Autodesk Maya".to_string(),
                config_search_string: "maya".to_string(),
                kind: AppKind::Maya,
            },
            ThreeDsMax: App {
                default_project_name: "3ds Max Project".to_string(),
                drp_client_id: "1016395801402036315".to_string(),
                process_search_string: " Autodesk 3ds Max".to_string(),
                config_search_string: "threedsmax".to_string(),
                kind: AppKind::ThreeDsMax,
            },
            AfterEffects: App {
                default_project_name: "After Effects Project".to_string(),
                drp_client_id: "1016395319522623539".to_string(),
                process_search_string: "Adobe After Effects".to_string(),
                config_search_string: "after_effects".to_string(),
                kind: AppKind::AfterEffects,
            },
            PremierePro: App {
                default_project_name: "Premiere Pro Project".to_string(),
                drp_client_id: "1016396017832300555".to_string(),
                process_search_string: "Adobe Premiere Pro".to_string(),
                config_search_string: "premiere_pro".to_string(),
                kind: AppKind::PremierePro,
            },
            Illustrator: App {
                default_project_name: "Illustrator Project".to_string(),
                drp_client_id: "1017491707819991071".to_string(),
                process_search_string: "Adobe Illustrator".to_string(),
                config_search_string: "illustrator".to_string(),
                kind: AppKind::Illustrator,
            },
            Photoshop: App {
                default_project_name: "Photoshop Project".to_string(),
                drp_client_id: "1017493347415371917".to_string(),
                process_search_string: "Photoshop".to_string(),
                config_search_string: "photoshop".to_string(),
                kind: AppKind::Photoshop,
            },
            Indesign: App {
                default_project_name: "Indesign Project".to_string(),
                drp_client_id: "1017493794456862781".to_string(),
                process_search_string: "Indesign".to_string(),
                config_search_string: "indesign".to_string(),
                kind: AppKind::Indesign,
            },
            DavinciResolve: App {
                default_project_name: "Davinci Resolve Project".to_string(),
                drp_client_id: "1016371757722128516".to_string(),
                process_search_string: "Davinci Resolve".to_string(),
                config_search_string: "davinci_resolve".to_string(),
                kind: AppKind::DavinciResolve,
            },
            Clarisse: App {
                default_project_name: "Isotropix Clarisse Project".to_string(),
                drp_client_id: "1016372248128532500".to_string(),
                process_search_string: "Isotropix Clarisse".to_string(),
                config_search_string: "clarisse".to_string(),
                kind: AppKind::Clarisse,
            },
            Marvelous: App {
                default_project_name: "Marvelous Designer Project".to_string(),
                drp_client_id: "1016372646046351423".to_string(),
                process_search_string: "Marvelous Designer".to_string(),
                config_search_string: "marvelous".to_string(),
                kind: AppKind::Marvelous,
            },
            Calvary: App {
                default_project_name: "Cavalry Project".to_string(),
                drp_client_id: "1016374130037243974".to_string(),
                process_search_string: "calvary.exe".to_string(),
                config_search_string: "calvary".to_string(),
                kind: AppKind::Calvary,
            },
            Ableton: App {
                default_project_name: "Ableton Project".to_string(),
                drp_client_id: "1016375030227161150".to_string(),
                process_search_string: "Ableton Live".to_string(),
                config_search_string: "ableton".to_string(),
                kind: AppKind::Ableton,
            },
            FL: App {
                default_project_name: "FL Studio Project".to_string(),
                drp_client_id: "1016393561140375712".to_string(),
                process_search_string: "FL Studio".to_string(),
                config_search_string: "fl".to_string(),
                kind: AppKind::FL,
            },
        }
    }
}

impl Apps {
    pub fn find_app(&self, str: &String) -> Option<&App> {
        for app in self.as_iter() {
            if str.contains(&app.process_search_string) {
                return Some(&app);
            }
        }
        None
    }
    pub fn find_config_app(&self, str: &str) -> Option<&App> {
        for app in self.as_iter() {
            if str == app.config_search_string {
                return Some(app);
            }
        }
        return None;
    }
}

impl App {
    pub fn parse(&self, window_title: &String) -> String {
        match self.kind {
            AppKind::C4d => {
                let mut end_i: usize = window_title.len();
                let mut start_i: usize = 0;
                for (i, char) in window_title.chars().enumerate() {
                    if char == '[' {
                        start_i = i;
                        match window_title.find(".c4d]") {
                            Some(i) => end_i = i as usize,
                            None => match window_title.find(" - Main") {
                                Some(i) => end_i = i as usize - 1,
                                None => end_i = window_title.len() - 1,
                            },
                        };
                    }
                }
                return window_title[start_i + 1..end_i].to_string();
            }
            AppKind::Maya => {
                let match_index = match window_title.as_str().find(".mb* - Autodesk Maya") {
                    Some(i) => i,
                    None => match window_title.as_str().find(".mb - Autodesk Maya") {
                        Some(i) => i + 4,
                        None => {
                            return String::from("Autodesk Maya Project");
                        }
                    },
                };
                return window_title[..match_index as usize].to_string();
            }
            AppKind::AfterEffects => {
                let mut proj_name = String::new();
                for i in (0..window_title.len() - 2).rev() {
                    if window_title.chars().nth(i).unwrap() == '\\' {
                        for char in window_title.chars().skip(i + 1) {
                            proj_name.push(char);
                        }
                        return proj_name;
                    }
                }
                if proj_name == "" {
                    proj_name = window_title.clone();
                }
                proj_name
            }
            AppKind::ThreeDsMax => self.default_project_name.clone(),
            AppKind::PremierePro => self.default_project_name.clone(),
            AppKind::Illustrator => self.default_project_name.clone(),
            AppKind::Photoshop => self.default_project_name.clone(),
            AppKind::Indesign => self.default_project_name.clone(),
            AppKind::DavinciResolve => self.default_project_name.clone(),
            AppKind::Clarisse => self.default_project_name.clone(),
            AppKind::Marvelous => self.default_project_name.clone(),
            AppKind::Calvary => self.default_project_name.clone(),
            AppKind::Ableton => self.default_project_name.clone(),
            AppKind::FL => self.default_project_name.clone(),
        }
    }
}
