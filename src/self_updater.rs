use windows::w;

pub fn try_update() -> Result<(), Box<dyn std::error::Error>> {
    let build = self_update::backends::github::Update::configure()
        .repo_owner("djkato")
        .repo_name("DRP_Creative")
        .bin_name("DRP_Creative")
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?;

    let latest_release = build.get_latest_release()?;
    if self_update::version::bump_is_greater(
        build.current_version().as_str(),
        latest_release.version.as_str(),
    )? {
        let body: windows::core::HSTRING = format!(
            "Found update! \nCurrent version: {}\nFound Version: {}\nUpdate?",
            build.current_version(),
            latest_release.version
        )
        .into();
        if let Ok(response) = win_msgbox::MessageBox::<win_msgbox::OkayCancel>::new(body.as_ptr())
            .title(w!("DRP Creative Updater").as_ptr())
            .show()
        {
            match response {
                win_msgbox::OkayCancel::Cancel => {}
                win_msgbox::OkayCancel::Okay => {
                    build.update()?;
                    win_msgbox::MessageBox::<win_msgbox::Okay>::new(
                        w!("Update successful, restart app to apply").as_ptr(),
                    )
                    .title(w!("DRP Creative").as_ptr())
                    .icon(win_msgbox::Icon::Information)
                    .show()
                    .expect("Failed to show update successful window");
                    return Ok(());
                }
            }
        };
    } else {
        return Err("No updates found".into());
    }
    Err("Something got skipped".into())
}
