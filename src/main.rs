#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate self_update;

pub mod app;
pub mod config;
pub mod program_status;
pub mod self_updater;
pub mod tray_icon;
use crate::config::{Config, IncludeExclude};
use crate::program_status::*;
use crate::self_updater::try_update;
use app::{App, Apps};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};
fn main() {
    match try_update() {
        Ok(_) => {}
        Err(e) => println!("Failed to update! {e}"),
    }
    let apps = Apps::construct_apps();
    let mut config = Config::default();

    config.load_from_file();

    let timeout = time::Duration::from_millis(5000);

    let (tray_receiver, _tray) = tray_icon::create_tray();

    let mut prev_project_name = String::new();
    let mut project_name = String::new();
    let mut start_time = 0;
    let mut drp_is_running = false;
    let mut discord_client: Option<DiscordIpcClient> = None;
    let mut current_app_option: Option<&App> = None;
    'main: loop {
        if let Some(current_app) = current_app_option {
            if let Some(real_project_name) = is_program_still_running(&current_app) {
                project_name = match config.should_list_include_or_exclude {
                    IncludeExclude::Exclude => real_project_name.clone(),
                    IncludeExclude::Include => current_app.default_project_name.clone(),
                };
                for listed_word in &config.keywords_list {
                    match &config.should_list_include_or_exclude {
                        &IncludeExclude::Exclude => {
                            if real_project_name.contains(listed_word.as_str()) {
                                if !&config.show_default_when_excluded {
                                    continue 'main;
                                }
                                project_name = current_app.default_project_name.clone();
                            } else {
                                project_name = real_project_name.clone();
                            }
                        }
                        &IncludeExclude::Include => {
                            if real_project_name.contains(listed_word.as_str()) {
                                project_name = real_project_name.clone();
                            } else {
                                if !&config.show_default_when_excluded {
                                    continue 'main;
                                }
                                project_name = current_app.default_project_name.clone();
                            }
                        }
                    }
                }
                //If discord client isn't connected create and conenct it
                if !drp_is_running {
                    //make a new client with current_app ID
                    discord_client =
                        match DiscordIpcClient::new(&current_app.drp_client_id.as_str()) {
                            Ok(ok) => Some(ok),
                            Err(_err) => None,
                        };
                    if let Some(dc_client) = discord_client.as_mut() {
                        match dc_client.connect() {
                            Ok(_ok) => drp_is_running = true,
                            Err(_err) => drp_is_running = false,
                        }
                    } else {
                        panic!();
                    }
                    //only if new app is made refresh the start time
                    start_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("time went backwards")
                        .as_secs();
                }
                //if project name changed, update it
                if prev_project_name != project_name {
                    prev_project_name = project_name.clone();
                    let details = format!("Working on {}", &project_name.clone());
                    //update activity

                    let mut activity = activity::Activity::new()
                        .details(&details.as_str())
                        .assets(
                            activity::Assets::new().large_image(current_app.drp_client_id.as_str()),
                        )
                        .timestamps(activity::Timestamps::new().start(start_time as i64));
                    let state;
                    if !config.hide_portfolio_row {
                        state = format!("Portfolio: {}", &config.portfolio_link);
                        activity = activity.state(state.as_str());
                    }
                    //if discord client exists, update status
                    if let Some(dc) = discord_client.as_mut() {
                        match dc.set_activity(activity) {
                            Ok(_) => (),
                            Err(e) => {
                                dbg!(e);
                            }
                        }
                    }
                }
            }
            //if program no longer running
            else {
                current_app_option = None;
                prev_project_name = "".to_string();
            }
            //respond to tray icon messages
            match tray_receiver.try_recv() {
                Ok(msg) => match msg {
                    tray_icon::Message::AddProjectToList => {
                        if let Some(real_project_name) = is_program_still_running(&current_app) {
                            config.add_project(&real_project_name);
                        }
                    }
                    tray_icon::Message::RemoveProjectFromList => {
                        if let Some(real_project_name) = is_program_still_running(&current_app) {
                            config.remove_project(&real_project_name);
                        }
                    }
                    tray_icon::Message::Quit => {
                        println!("qiuitting!");
                        exit(0)
                    }
                    tray_icon::Message::OpenOptionsFile => {
                        let _ = std::process::Command::new("notepad")
                            .arg("drp_config.toml")
                            .current_dir("./")
                            .spawn();
                    }
                },
                Err(_err) => (),
            }
        } else {
            //respond to tray icon messages
            match tray_receiver.try_recv() {
                Ok(msg) => match msg {
                    tray_icon::Message::Quit => exit(0),
                    _ => (),
                },
                Err(_err) => (),
            }
            //If nothing is running try to find it
            if let Some(proj_name) = get_running_program(&apps) {
                let temp_curr_app: &App;
                (temp_curr_app, project_name) = proj_name;
                current_app_option = Some(temp_curr_app);
            } else {
                current_app_option = None;
            }
            if drp_is_running {
                match discord_client
                    .as_mut()
                    .expect("Somehow it dissapeared?")
                    .close()
                {
                    Ok(_ok) => drp_is_running = false,
                    Err(_err) => (),
                }
            }
        }
        thread::sleep(timeout);
    }
}
