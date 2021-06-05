use clap::{App, AppSettings, Arg};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use std::error::Error;

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load()?;
    let theme = &ColorfulTheme::default();

    let matches = App::new("kino")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            App::new("play").about("Search and play a media").arg(
                Arg::with_name("name")
                    .required(true)
                    .help("Name of the media to search for"),
            ),
        )
        .subcommand(App::new("init").about("Setup config"))
        .get_matches();

    match matches.subcommand() {
        ("play", Some(matches)) => {
            let search = matches.value_of("name").unwrap();
            play(search, &mut config, theme).await?;
        }
        ("init", Some(_)) => {
            init(&mut config, theme).await?;
        }
        ("", None) => {}
        _ => unreachable!(),
    }

    Ok(())
}

// FIXME: renew token when it's expired (it might)
async fn retreive_token<'a>(
    config: &'a mut config::Config,
    theme: &ColorfulTheme,
) -> Result<&'a str, Box<dyn Error>> {
    if config.token().is_none() {
        let username: String;
        let password: String;
        if !api::auth::admin_exists().await? {
            println!("No admin exists yet.\nRegistering a new user:");
            username = Input::with_theme(theme)
                .with_prompt("Username")
                .interact_text()?;
            password = Password::with_theme(theme)
                .with_prompt("Password")
                .interact()?;
            println!("registering...");
            api::auth::register(&username, &password).await?;
        } else {
            println!("No token available, please login.");
            username = Input::with_theme(theme)
                .with_prompt("Username")
                .interact_text()?;
            password = Password::with_theme(theme)
                .with_prompt("Password")
                .interact()?;
        }
        println!("logging in...");
        let tok = api::auth::login(&username, &password).await?;
        config.set_token(Some(tok));
        config.store()?;
    }
    let tok = config.token().as_ref().unwrap();
    Ok(tok)
}

fn retreive_player<'a>(
    config: &'a mut config::Config,
    theme: &ColorfulTheme,
) -> Result<&'a str, Box<dyn Error>> {
    if config.player().is_none() {
        println!("No player setup.");
        let player = Input::with_theme(theme)
            .with_prompt("Enter your media player's command:")
            .interact_text()?;

        config.set_player(Some(player));
        config.store()?;
    }
    let player = config.player().as_ref().unwrap();
    Ok(player)
}

async fn init(config: &mut config::Config, theme: &ColorfulTheme) -> Result<(), Box<dyn Error>> {
    retreive_token(config, theme).await?;
    retreive_player(config, theme)?;
    Ok(())
}

async fn play(
    search: &str,
    config: &mut config::Config,
    theme: &ColorfulTheme,
) -> Result<(), Box<dyn Error>> {
    let token = retreive_token(config, theme).await?;

    let results = api::search(search, &token).await?;

    // select a result
    let selection = Select::with_theme(theme)
        .with_prompt("Select a media:")
        .items(&results.iter().map(|item| &item.name).collect::<Vec<_>>())
        .default(0)
        .interact()?;
    let selected_id = results[selection].id;
    // let selected_media = api::media::get(selected_id, &token).await?;
    let selected_media_info = api::media::info(selected_id, &token).await?;
    let versions = selected_media_info.versions;

    // select a file
    let version_selection = if versions.len() == 1 {
        0
    } else {
        Select::with_theme(theme)
            .with_prompt("Select a version:")
            .default(0)
            .items(
                &versions
                    .iter()
                    .map(|item| &item.display_name)
                    .collect::<Vec<_>>(),
            )
            .interact()?
    };
    let selected_file = &versions[version_selection].file;
    let selected_file_name = std::path::PathBuf::from(selected_file)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("file".to_string());

    // play file
    let player = retreive_player(config, theme)?;
    println!("Playing {} with {}...", selected_file_name, player);
    std::process::Command::new(player)
        .arg(selected_file)
        .output()?;
    Ok(())
}
