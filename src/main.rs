use clap::Parser;
use interface::{AllArgs, Selector, SubCommands, CLI};
use std::process::Command;
mod interface;

fn main() {
    let cli = CLI::parse();

    match cli.command {
        SubCommands::SetVolume { allargs, volume } => {
            volume_action("setvolume", allargs, Some(volume))
        }
        SubCommands::MuteVolume { allargs } => volume_action("mute", allargs, None),
        SubCommands::GenerateExampleConfig {} => {}
    }
}

fn volume_action(action: &str, allargs: AllArgs, volume: Option<String>) {
    let selector = match allargs.select {
        Selector::Output => "@DEFAULT_SINK@",
        Selector::Input => "@DEFAULT_SOURCE@",
    };

    let selector_name = match allargs.select {
        Selector::Output => "Output",
        Selector::Input => "Input",
    };
    let mut wpctl_volume_bind = Command::new("sh");
    let wpctl_volume_cmd = wpctl_volume_bind
        .arg("-c")
        .arg(format!("wpctl get-volume {} | cut -f2 -d' '", selector));
    let old_getvolume: f32 = String::from_utf8(
        wpctl_volume_cmd
            .output()
            .expect("wpctl failed to get volume")
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string()
    .parse()
    .expect("Failed to parse as float");
    let old_volume_int = old_getvolume * 100.0;

    match action {
        "mute" => {
            let notify_args = match allargs.notify {
                Some(e) => e,
                None => format!("--icon=audio-volume-high -u low -t 1000 -e -h string:synchronous:audio-volume 'Audio volume' '{} Toggled'", selector_name),
            };

            Command::new("sh")
                .arg("-c")
                .arg(format!("wpctl set-mute {} toggle", selector))
                .output()
                .expect("wpctl failed to set mute");

            //TODO: Make this optional in the future
            Command::new("sh")
                .arg("-c")
                .arg(format!("notify-send {}", notify_args))
                .output()
                .expect("notify-send failed to set notification");
        }
        "setvolume" => {
            let volume_unwrap = volume.unwrap();
            if old_volume_int != 100.0 {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("wpctl set-volume {} {}", selector, volume_unwrap))
                    .output()
                    .expect("wpctl failed to set mute");
            }

            let getvolume: f32 = String::from_utf8(
                wpctl_volume_cmd
                    .output()
                    .expect("wpctl failed to get volume")
                    .stdout,
            )
            .unwrap()
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse as float");
            let volume_int = getvolume * 100.0;

            let notify_args = match allargs.notify {
                Some(e) => e,
                None => format!("--icon=audio-volume-high -u low -t 1000 -e -h int:value:{} -h string:synchronous:audio-volume 'Audio volume' 'Set {} to {}%'", volume_int, selector_name, volume_int),
            };

            //TODO: Make this optional in the future
            Command::new("sh")
                .arg("-c")
                .arg(format!("notify-send {}", notify_args))
                .output()
                .expect("notify-send failed to set notification");
        }
        _ => eprint!("It must contain setvolume or mute"),
    };
}
