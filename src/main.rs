#![windows_subsystem = "windows"] //UNCOMMENT ONLY WHEN BUILDING FOR RELEASE TO NOT SHOW TERMINAL WINDOW!
pub mod app;
pub mod config;
pub mod program_status;
pub mod tray_icon;
use crate::config::Config;
use crate::program_status::*;
use app::{App, Apps};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};
fn main() {
    let apps = Apps::construct_apps();
    let mut config = Config::load();

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
                //if project name includes filtered words, use default project name
                for excluded_word in &config.exclude_keywords_list {
                    if real_project_name.contains(excluded_word.as_str()) {
                        project_name = current_app.default_project_name.clone();
                    } else {
                        project_name = real_project_name.clone();
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
                    let state = format!("Portfolio: {}", &config.portfolio_link);
                    let activity = activity::Activity::new()
                        .state(state.as_str())
                        .details(&details.as_str())
                        .assets(
                            activity::Assets::new().large_image(current_app.drp_client_id.as_str()),
                        )
                        .timestamps(activity::Timestamps::new().start(start_time as i64));
                    //if discord client exists, update status
                    if let Some(dc) = discord_client.as_mut() {
                        match dc.set_activity(activity) {
                            _ => (),
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
                    tray_icon::Message::AnonymiseProject => {
                        //only exclude project name if it's not the default one
                        for app in apps.as_iter() {
                            if app.kind == current_app.kind {
                                if app.default_project_name != project_name {
                                    config.exclude_project(&project_name);
                                }
                            }
                        }
                    }
                    tray_icon::Message::Quit => break 'main,
                },
                Err(_err) => (),
            }
        } else {
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
