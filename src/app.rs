use crate::program_status::get_running_windows_titles;
use lazy_static::lazy_static;
use regex::Regex;
extern crate lazy_static;
#[derive(Debug)]
pub struct App {
    pub kind: AppKind,
    pub default_project_name: String,
    pub drp_client_id: String,
    pub process_search_string: String,
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
    Cavalry,
    Ableton,
    FL,
    Blender,
    Audition,
    SubstancePainter,
    SubstanceDesigner,
    Vegas,
    ZBrush,
    Darktable,
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
    Cavalry: App,
    Ableton: App,
    FL: App,
    Blender: App,
    Audition: App,
    SubstancePainter: App,
    SubstanceDesigner: App,
    Vegas: App,
    ZBrush: App,
    Darktable: App,
}
impl Apps {
    pub fn as_iter(&self) -> [&App; 21] {
        let APPS: [&App; 21] = [
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
            &self.Cavalry,
            &self.Ableton,
            &self.FL,
            &self.Blender,
            &self.Audition,
            &self.SubstancePainter,
            &self.SubstanceDesigner,
            &self.Vegas,
            &self.ZBrush,
            &self.Darktable,
        ];
        APPS
    }
}
impl AppKind {
    pub fn as_iter() -> [AppKind; 21] {
        let APPKINDS: [AppKind; 21] = [
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
            AppKind::Cavalry,
            AppKind::Ableton,
            AppKind::FL,
            AppKind::Blender,
            AppKind::Audition,
            AppKind::SubstancePainter,
            AppKind::SubstanceDesigner,
            AppKind::Vegas,
            AppKind::ZBrush,
            AppKind::Darktable,
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
                kind: AppKind::C4d,
            },
            Maya: App {
                default_project_name: "Autodesk Maya Project".to_string(),
                drp_client_id: "1016369550276694106".to_string(),
                process_search_string: "Autodesk Maya".to_string(),
                kind: AppKind::Maya,
            },
            ThreeDsMax: App {
                default_project_name: "3ds Max Project".to_string(),
                drp_client_id: "1016395801402036315".to_string(),
                process_search_string: " Autodesk 3ds Max".to_string(),
                kind: AppKind::ThreeDsMax,
            },
            AfterEffects: App {
                default_project_name: "After Effects Project".to_string(),
                drp_client_id: "1016395319522623539".to_string(),
                process_search_string: "Adobe After Effects".to_string(),
                kind: AppKind::AfterEffects,
            },
            PremierePro: App {
                default_project_name: "Premiere Pro Project".to_string(),
                drp_client_id: "1016396017832300555".to_string(),
                process_search_string: "Adobe Premiere Pro".to_string(),
                kind: AppKind::PremierePro,
            },
            Illustrator: App {
                default_project_name: "Illustrator Project".to_string(),
                drp_client_id: "1017491707819991071".to_string(),
                process_search_string: "Illustrator.exe".to_string(),
                kind: AppKind::Illustrator,
            },
            Photoshop: App {
                default_project_name: "Photoshop Project".to_string(),
                drp_client_id: "1017493347415371917".to_string(),
                process_search_string: "Photoshop.exe".to_string(),
                kind: AppKind::Photoshop,
            },
            Indesign: App {
                default_project_name: "Indesign Project".to_string(),
                drp_client_id: "1017493794456862781".to_string(),
                process_search_string: "Indesign".to_string(),
                kind: AppKind::Indesign,
            },
            DavinciResolve: App {
                default_project_name: "Davinci Resolve Project".to_string(),
                drp_client_id: "1016371757722128516".to_string(),
                process_search_string: "DaVinci Resolve - ".to_string(),
                kind: AppKind::DavinciResolve,
            },
            Clarisse: App {
                default_project_name: "Isotropix Clarisse Project".to_string(),
                drp_client_id: "1016372248128532500".to_string(),
                process_search_string: "Isotropix Clarisse".to_string(),
                kind: AppKind::Clarisse,
            },
            Marvelous: App {
                default_project_name: "Marvelous Designer Project".to_string(),
                drp_client_id: "1016372646046351423".to_string(),
                process_search_string: "Marvelous Designer".to_string(),
                kind: AppKind::Marvelous,
            },
            Cavalry: App {
                default_project_name: "Cavalry Project".to_string(),
                drp_client_id: "1016374130037243974".to_string(),
                process_search_string: "Cavalry.exe".to_string(),
                kind: AppKind::Cavalry,
            },
            Ableton: App {
                default_project_name: "Ableton Project".to_string(),
                drp_client_id: "1016375030227161150".to_string(),
                process_search_string: "Ableton Live".to_string(),
                kind: AppKind::Ableton,
            },
            FL: App {
                default_project_name: "FL Studio Project".to_string(),
                drp_client_id: "1016393561140375712".to_string(),
                process_search_string: "FL Studio".to_string(),
                kind: AppKind::FL,
            },
            Blender: App {
                kind: AppKind::Blender,
                default_project_name: "Blender Project".to_string(),
                drp_client_id: "1017878734746963978".to_string(),
                process_search_string: "BlenderGLEW".to_string(),
            },
            Audition: App {
                kind: AppKind::Audition,
                default_project_name: "Audition Project".to_string(),
                drp_client_id: "1017879756282286153".to_string(),
                process_search_string: "Adobe Audition.exe".to_string(),
            },
            SubstancePainter: App {
                kind: AppKind::SubstancePainter,
                default_project_name: "Substance Painter Project".to_string(),
                drp_client_id: "1017881633296232578".to_string(),
                process_search_string: "Substance 3D Painter".to_string(),
            },
            SubstanceDesigner: App {
                kind: AppKind::SubstanceDesigner,
                default_project_name: "Substance Designer Project".to_string(),
                drp_client_id: "1017881386583080971".to_string(),
                process_search_string: "Substance Designer".to_string(),
            },
            Vegas: App {
                kind: AppKind::Vegas,
                default_project_name: "Vegas Project".to_string(),
                drp_client_id: "1017882355723153448".to_string(),
                process_search_string: "VEGAS Pro".to_string(),
            },
            ZBrush: App {
                kind: AppKind::Vegas,
                default_project_name: "ZBrush Project".to_string(),
                drp_client_id: "1112734356855865425".to_string(),
                process_search_string: "ZBrush".to_string(),
            },
            Darktable: App {
                kind: AppKind::Darktable,
                default_project_name: "Darktable Project".to_string(),
                drp_client_id: "1116027286110609459".to_string(),
                process_search_string: "darktable.exe".to_string(),
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
                        if let Some(curr_end_i) = window_title.rfind("] - ") {
                            end_i = curr_end_i;
                        } else {
                            return self.default_project_name.clone();
                        }
                    }
                }
                return window_title[start_i + 1..end_i].to_string();
            }
            AppKind::Maya => {
                let match_index = match window_title.as_str().find(".mb* - Autodesk Maya") {
                    Some(i) => i + 5,
                    None => match window_title.as_str().find(".mb - Autodesk Maya") {
                        Some(i) => i + 4,
                        None => match window_title.as_str().find(" - Autodesk Maya") {
                            Some(i) => i,
                            None => {
                                return self.default_project_name.clone();
                            }
                        },
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
            AppKind::ThreeDsMax => {
                let match_index = match window_title.as_str().find(".max* - Autodesk 3ds Max") {
                    Some(i) => i + 5,
                    None => match window_title.as_str().find(".max - Autodesk 3ds Max") {
                        Some(i) => i + 4,
                        None => match window_title.as_str().find(" - Autodesk 3ds Max") {
                            Some(i) => i,
                            None => {
                                return self.default_project_name.clone();
                            }
                        },
                    },
                };
                return window_title[..match_index as usize].to_string();
            }
            AppKind::DavinciResolve => {
                let match_index = match window_title.as_str().find("DaVinci Resolve -") {
                    Some(i) => 17 as usize,
                    None => 0 as usize,
                };
                let mut with_project_extention = window_title.clone();
                with_project_extention.push_str(".drp");
                return with_project_extention[match_index as usize..].to_string();
            }
            AppKind::PremierePro => {
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
            AppKind::Illustrator => {
                let running_windows_titles = unsafe { get_running_windows_titles() };
                lazy_static! {
                    static ref REG_AI: Regex =
                        Regex::new(r".{0,} @ (\d{0,}\.\d{0,}|\d{0,}) % \(\w{3,4}\W\w{0,}\b\)")
                            .unwrap();
                }
                let mut match_index = self.default_project_name.len();
                let mut matching_title = self.default_project_name.clone();

                for titles in running_windows_titles {
                    if REG_AI.is_match(&titles) {
                        if let Some(i) = titles.find("@") {
                            matching_title = titles.clone();
                            match_index = i;
                        }
                    }
                }
                let out_string = matching_title[..match_index].to_string();
                out_string
            }
            AppKind::Photoshop => {
                let running_windows_titles = unsafe { get_running_windows_titles() };
                lazy_static! {
                    static ref REG_PSD: Regex =
                        Regex::new(r".{0,}@ (\d{0,}|\d{0,}.\d{0,})% \(.{0,}, \w{0,}\W.{0,}\)")
                            .unwrap();
                }
                let mut match_index = self.default_project_name.len();
                let mut matching_title = self.default_project_name.clone();

                for titles in running_windows_titles {
                    if REG_PSD.is_match(&titles) {
                        if let Some(i) = titles.find("@") {
                            matching_title = titles.clone();
                            match_index = i;
                        }
                    }
                }
                let out_string = matching_title[..match_index].to_string();
                out_string
            }
            AppKind::Indesign => self.default_project_name.clone(),
            AppKind::Clarisse => {
                let match_index = match window_title.as_str().find(".ple* - Isotropix Clarisse") {
                    Some(i) => i + 5,
                    None => match window_title.as_str().find(".ple - Isotropix Clarisse") {
                        Some(i) => i + 4,
                        None => match window_title.as_str().find(" - Isotropix Clarisse") {
                            Some(i) => i,
                            None => {
                                return self.default_project_name.clone();
                            }
                        },
                    },
                };
                return window_title[..match_index as usize].to_string();
            }
            AppKind::Marvelous => self.default_project_name.clone(),
            AppKind::Cavalry => {
                let running_windows_titles = unsafe { get_running_windows_titles() };
                lazy_static! {
                    static ref REG_CV: Regex =
                        Regex::new(r"Project: .{0,}Scene: .{0,}\.cv ").unwrap();
                }
                let mut match_index = 0;
                let mut matching_title = self.default_project_name.clone();

                for titles in running_windows_titles {
                    if REG_CV.is_match(&titles) {
                        if let Some(i) = titles.rfind(r"/") {
                            matching_title = titles.clone();
                            match_index = i + 1;
                        }
                    }
                }
                let out_string = matching_title[match_index..].to_string();
                out_string
            }
            AppKind::Ableton => {
                let match_index = match window_title.as_str().find(" - Ableton Live") {
                    Some(i) => i,
                    None => return self.default_project_name.clone(),
                };
                return window_title[..match_index as usize].to_string();
            }
            AppKind::FL => {
                let match_index = match window_title.as_str().find(" - FL Studio") {
                    Some(i) => i,
                    None => return self.default_project_name.clone(),
                };
                return window_title[..match_index as usize].to_string();
            }
            AppKind::Blender => {
                let running_windows_titles = unsafe { get_running_windows_titles() };
                lazy_static! {
                    static ref REG_BL: Regex =
                        Regex::new(r"Blender \[.{0,}\\\w{0,}\.blend]").unwrap();
                }
                let mut match_index = 0;
                let mut matching_title = self.default_project_name.clone();

                for titles in running_windows_titles {
                    if REG_BL.is_match(&titles) {
                        if let Some(i) = titles.rfind(r"\") {
                            matching_title = titles.clone();
                            match_index = i + 1;
                        }
                    }
                }
                let end_i = matching_title.len();
                let out_string = matching_title[match_index..end_i - 1].to_string();
                out_string
            }
            AppKind::Audition => self.default_project_name.clone(),
            AppKind::SubstancePainter => self.default_project_name.clone(),
            AppKind::SubstanceDesigner => self.default_project_name.clone(),
            AppKind::Vegas => {
                let match_index = match window_title.as_str().find(".veg* - VEGAS Pro") {
                    Some(i) => i + 5,
                    None => match window_title.as_str().find(".veg - VEGAS Pro") {
                        Some(i) => i + 4,
                        None => match window_title.as_str().find(" - VEGAS Pro") {
                            Some(i) => i,
                            None => {
                                return self.default_project_name.clone();
                            }
                        },
                    },
                };
                return window_title[..match_index as usize].to_string();
            }
            AppKind::ZBrush => self.default_project_name.clone(),
            AppKind::Darktable => self.default_project_name.clone(),
        }
    }
}
