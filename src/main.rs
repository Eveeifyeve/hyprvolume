use clap::Parser;
use interface::{CLI, SubCommands, AllArgs};
use std::{process::Command, io::{Write, stdout}};
mod interface; 



fn main() {
    let cli = CLI::parse();

    match cli.command {
        SubCommands::SetVolume {allargs} => volume_action("setvolume", allargs),
        SubCommands::MuteVolume {allargs} => volume_action("mute", allargs),
        SubCommands::GenerateExampleConfig {} => {}
    }
}


fn volume_action(action: &str, allargs: AllArgs) {
    println!("function call");

    let selector = "@DEFAULT_SINK@";
    let wpctl_volume = Command::new("sh")
        .arg("-c")
        .arg(format!("wpctl get-volume {} | cut -f2 -d' '", selector))
        .output()
        .expect("wpctl failed to get volume");
    let getvolume = String::from_utf8(wpctl_volume.stdout).unwrap().trim().to_string();
    let getvolume_parse: f32 = getvolume.parse().expect("Failed to parse as a interger");
    let volume_int = getvolume_parse * 100.0;
    println!("volume: {}", volume_int);

    let selector_name = match selector {
        "@DEFAULT_SINK@" => "Output",
        "@DEFAULT_SOURCE" => "Input",
        _ => "@DEFAULT_SINK"
    };

    match allargs.notify {
        None => println!("Notify"),
        Some(_stuff) => println!("Notify"),
    }

    match action {
        "mute" => {
            println!("action work");
            Command::new("sh")
                .arg("-c")
                .arg(format!("wpctl set-mute {} toggle", selector))
                .output()
                .expect("wpctl failed to set mute");

            //TODO: Make this optional in the future
            Command::new("sh")
                .arg("-c")
                .arg(format!("notify-send --icon=audio-volume-high -u low -t 1000 -e -h string:synchronous:audio-volume 'Audio volume' '{} Toggled'", selector_name))
                .output()
                .expect("notify-send failed to set notification");
        }
        "setvolume" => {}
        _ => eprint!("It must contain setvolume or mute")
    };
}



