#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::UserDirs;
use fltk::{prelude::*, *};

const ICON: &str = r#"
<svg width='800' height='800' xmlns='http://www.w3.org/2000/svg'>
    <rect height='760' width='760' x='20' y='20' fill='#e5e5e5' />
    <rect stroke='#000' height='350' width='500' x='80' y='180'  fill='#fff'/>
</svg>
"#;

fn main() {
    let app = app::App::default();

    let mut picture_folder = "".to_owned();

    if let Some(user_folder) = UserDirs::new() {
        if let Some(picture_path) = user_folder.picture_dir() {
            if let Some(picture_path) = picture_path.to_str() {
                picture_folder = picture_path.to_owned()
            }
        }
    }

    let mut main_window = window::Window::default()
        .with_size(350, 220)
        .with_label("Hide Default Folders");

    let icon = image::SvgImage::from_data(ICON).unwrap();

    main_window.set_icon(Some(icon));

    let mut vpack = group::Pack::default()
        .with_size(330, 190)
        .with_pos(10, 10);
    
    vpack.set_spacing(7);

    frame::Frame::default()
        .with_size(330, 25)
        .with_label("Where to save screenshots?")
        .with_align(enums::Align::Left | enums::Align::Inside);
    
    let mut flex = group::Flex::default()
        .with_size(280, 25)
        .with_type(group::FlexType::Row);
    
    let mut input = input::Input::default()
        .with_align(enums::Align::TopLeft);

    input.set_value(&picture_folder);

    let mut button_select = button::Button::default()
        .with_label("Select");

    flex.set_size(&mut button_select, 60);
    flex.end();
    
    frame::Frame::default()
        .with_size(330, 25)
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_label("What duration to take screenshot?");
 
    let mut time_input = input::Input::default()
        .with_size(330, 25);

    time_input.set_value("5 minutes");

    let hide_checkbutton = button::CheckButton::default()
        .with_size(0, 25)
        .with_label("Hide before taking screenshots");

    hide_checkbutton.set_checked(true);

    group::Flex::default()
        .with_size(320, 25);

    let mut button_start = button::Button::default()
        .with_label("Start Screenshot");

    let mut button_hide = button::Button::default()
        .with_label("Hide Window");
    
    flex.end();    
    vpack.end();

    main_window.end();
    main_window.show();

    let (s, r) = app::channel();
    button_select.emit(s, "dialog");
    button_start.emit(s, "print");
    button_hide.emit(s, "hide");
    
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                "dialog" => {
                    let mut dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseDir);
                    dialog.show();
                    let path = dialog.filename();
                    if let Some(p) = path.to_str() {            
                        println!("{}", p);                        
                        input.set_value(p);                       
                    };
                },
                "print" => {                    
                    match time_input.value().parse::<humantime::Duration>() {
                        Ok(duration) => {
                            println!("{}", duration);
                        },
                        _ => {
                            println!("Failed to do...");
                        }
                    }
                },
                "hide" => {
                    use tray_icon::{TrayIconBuilder, menu::Menu};

                    let tray_menu = Menu::new();
                    let tray_icon = TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu))
                        .with_tooltip("system-tray - tray icon library!")
                        .build()
                        .unwrap();
                }
                _ => {}
            }
        }
    }
}
