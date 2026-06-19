# **TF2 Web-Remote**

## _A simple yet fashionable way to control your personal TF2 Server instance_

If you like to play Team Fortress 2 with your friends on your own server you know what a pain it is to switch maps and manage bots/rules/modes etc., you simply can't do it in an easy and timely manner using the command line, especially when you don't wanna lose the momentum of the game.

TF2 Web Remote aims to solve this very problem with a sleek, fun and easy to use web interface!

## Setup
_Requires Python 3 to be installed on the host machine!_
The app comes pre-configured with the default `hostname` and `port numbers`, however you **will** need to edit the `config.yml` file with your own TF2 Server password if you set one up.
To start your Web Remote run `startLinux.sh` for Linux or macOS OR `startWin.bat` for Windows servers, do not run `main.py` directly as the script creates a virtual environment and checks for the necessary dependencies for the app to run correctly.

## Why a web interface?
Because it's fast and easy to setup, plus it can be accessed via the Steam Overlay's embedded web browser!

## What technologies were used?
Mainly HTML, CSS, JS and Python, but we do include a handy Rust-built utility (thank you [Stormix-dev](https://github.com/Stormix-dev)) that we used to download the thumbnails for the map selectors, it can absolutely be re-purposed for other purposes too.

## Is this app open source?
Yes! It's offered under the GPL 3.0 License.