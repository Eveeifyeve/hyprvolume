# hyprvolume
A Modern Volume control that uses `wpctl` & `` to control volume and send a notification

# Installation

[!NOTE]
> Please make the package and a PR that contains the installation instructions for your distro. if you want your distro contained here

## NixOS 

* Requires NixOS hyprland module to be enabled as this will install `wpctl`.
* Make sure you installed pipe wire or pulse audio

1. You can install by adding the following inputs:
```nix
inputs = {
    hyprvolume = {
        url = "github:eveeifyeve/hyprvolume";
        inputs.follows.hyprland = "hyprland";
    };
};
```
3. add the following to your packages:
```nix
inputs.hyprvolume.${pkgs.system}.packages.default
pkgs.libnotify
```

## Dependencies

* hyprland
* pipewire or pulseaudio
* libnotify (to get notifyed when your volune is changed)

## Manual 

To install manally make sure you have [rust]() & the deps above installed, simply clone this repo and run the following:
```sh
cargo install 
```
