# take-screenshots

Take-sreenshots is an easy-to-use tool that helps you take screenshots every few minutes. It runs on Windows, Linux and MacOS.

![take screenshots run on linuxmint](./screenshots/Take%20Screenshots.PNG)

## Features
 - customize output folder
 - customize time interval
 - customize output file name (supports [specified format string](https://docs.rs/chrono/latest/chrono/format/strftime/index.html))
 - auto minimize window before sreenshots
 - tray icon support (not supported yet!)

## Build instruction
### Ubuntu
```bash
sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev libdbus-1-dev pkg-config
```

## Other notes
Use following command to make take-screenshots executable:
```bash
chmod +x take-screenshots
```