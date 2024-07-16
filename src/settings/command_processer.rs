use log::info;
use web_time::Duration;

use crate::{
    app::Services,
    random::{none, Algorithm, ALGS},
};

use super::TFSetting;

pub fn process_command(settings: &mut TFSetting, services: &mut Services) {
    let command = settings.command.clone();

    if settings.command.contains(';') {
        settings.last_command.clone_from(&command);
        settings.command.clear();
    }

    if command.contains("level") {
        let new_level = match ALGS.into_iter().find(|alg| command.contains(alg.id)) {
            Some(alg) => alg,
            None => Algorithm {
                id: "None",
                version: "0",
                description: "this algorithm does not exist",
                lang: "mhh",
                random_function: &none,
            },
        };
        settings.change_level(new_level)
    }

    if command.contains("help") {
        info!("{}", command);
        let info = command
            .strip_prefix("help")
            .and_then(|x| x.strip_suffix(';'))
            .unwrap_or("");
        services
            .notifier
            .info(info)
            .set_closable(true)
            .set_duration(Some(Duration::from_secs_f32(30.0)));
    }
}
