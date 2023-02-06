#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::UserDirs;
use fltk::{prelude::*, *};
use chrono::{format::{strftime, Item}, Utc, DateTime, Local};
use screenshots::Screen;

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

    let mut main_window = window::SingleWindow::default()
        .with_size(410, 210)
        .with_label("Take Screenshots");

    let mut vpack = group::Pack::default()
        .with_size(390, 186)
        .center_of_parent();
    
    vpack.set_spacing(12);

    let mut flex = group::Flex::default()
        .with_size(280, 25)
        .with_type(group::FlexType::Row);

    let frame = frame::Frame::default()
        .with_size(330, 25)
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_label("Use Strategy");

    let mut screenshot_strategy = menu::Choice::default()
        .with_size(330, 25);

    screenshot_strategy.add_choice("screenshots-rs(Cross platform, default)|ksnip(Cross platform)|NirCmd(Only Windows)|Python MSS(Cross platform)");
    screenshot_strategy.set_value(0);

    flex.set_size(&frame, 85);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(280, 25)
        .with_type(group::FlexType::Row);

    let frame = frame::Frame::default()
        .with_label("Save at")
        .with_align(enums::Align::Left | enums::Align::Inside);
    
    let mut output_folder_input = input::Input::default()
        .with_align(enums::Align::TopLeft);

    output_folder_input.set_value(&picture_folder);

    let mut button_select = button::Button::default()
        .with_label("Select");

    flex.set_size(&frame, 55);
    flex.set_size(&mut button_select, 60);
    flex.end();

    let mut flex = group::Flex::default()
        .with_size(280, 25)
        .with_type(group::FlexType::Row);

    let frame = frame::Frame::default()
        .with_size(330, 25)
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_label("with name");
 
    let mut filename_format_input = input::Input::default()
        .with_size(330, 25);

    filename_format_input.set_value("ts_%Y_%m_%d-%H_%M_%S.png");  

    flex.set_size(&frame, 70);
    flex.end();
    
    let mut flex = group::Flex::default()
        .with_size(280, 25)
        .with_type(group::FlexType::Row);

    let frame_stop_at = frame::Frame::default()
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_label("Stop in");
 
    let mut stop_time_input = input::Input::default()
        .with_size(330, 25);

    stop_time_input.set_value("2 hours");

    frame::Frame::default()
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_label(", with duration");

    let mut duration_input = input::Input::default()
        .with_size(330, 25);

    duration_input.set_value("5 minutes");

    flex.set_size(&frame_stop_at, 50);
    flex.end();
    
    let mut flex = group::Flex::default()
        .with_size(320, 25);

    let mut button_start = button::Button::default()
        .with_label("Start screenshot");

    let minimize_checkbutton = button::CheckButton::default()
        .with_size(0, 25)
        .with_label("Minimize the window");

    minimize_checkbutton.set_checked(true);
    
    flex.set_size(&minimize_checkbutton, 150);
    flex.end();    
    vpack.end();

    main_window.end();
    main_window.show();

    let (s, r) = app::channel();
    button_select.emit(s, "dialog");
    button_start.emit(s, "start");
    // button_minimize.emit(s, "hide");
    
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                "dialog" => {
                    let mut dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseDir);
                    dialog.show();
                    let path = dialog.filename();
                    if let Some(p) = path.to_str() {            
                        println!("{p}");                        
                        output_folder_input.set_value(p);                       
                    };
                },
                "start" => {                    
                    let format = filename_format_input.value();

                    let durationt_time = duration_input.value().parse::<humantime::Duration>();
                    let stop_time = stop_time_input.value().parse::<humantime::Duration>();

                    let mut strftime_items = strftime::StrftimeItems::new(format.as_str());
                    let corret_format = !strftime_items.any(|item| matches!(item, Item::Error));
                    
                    let output_folder = output_folder_input.value();

                    match (durationt_time, stop_time, corret_format) {
                        (Ok(duration), Ok(stop), true) => {
                            let times = stop.as_millis() / duration.as_millis();
                            if stop.as_millis() > 0 {

                                let _job = std::thread::spawn(move || {

                                    for _ in 0..times {
                                        let screens = Screen::all().unwrap();
                                        if screens.len() > 1 {
                                            return;
                                        }
                            
                                        let screen = screens[0];
                                        let now: DateTime<Local> = Utc::now().into();     
                                        let filename = now.format(&format);  
                                        let image = screen.capture().unwrap();
                                        let buffer = image.buffer();
                                        std::fs::write(format!("{output_folder}/{filename}"), &buffer).unwrap();
                            
                                        std::thread::sleep(std::time::Duration::from_secs(duration.as_secs()))
                                    }

                                    s.send("activate");
                                });
                                button_start.deactivate();      
                            }                              
                        },
                        _ => {
                            println!("Failed to do...");
                        }
                    }
                },
                "hide" => {
                    main_window.iconize();                    
                },
                "activate" => {
                    button_start.activate();
                }
                _ => {}
            }
        }
    }
}