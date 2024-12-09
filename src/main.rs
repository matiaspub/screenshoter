use std::{env, fs};
use chrono::Utc;
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;

#[warn(clippy::all, clippy::pedantic)]

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;
    path.push(&screens_dir);

    fs::create_dir_all(path)?;

    if let Err(error) = grab(move |e| callback(e, &screens_dir)) {
        println!("Error: {error:?}");
    }

    Ok(())
}

fn callback(event: Event, screens_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::F12) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screen_dir: &String) {
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();
        
        let now = Utc::now();
        
        image.save(format!("{}/{}.png", screen_dir, now.format("%Y-%m-%d_%H:%M:%S"))).unwrap()
    }
}
