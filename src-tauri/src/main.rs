// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

fn switch_content(event: WindowMenuEvent, from: &str, to: &str) {
  let page_links: HashMap<&str, &str> = [
    ("chat", "search?q=Bing+AI&showconv=1"),
    ("image", "images/create"),
  ].iter().cloned().collect();

  let main_window = event.window();
  let menu_handle = main_window.menu_handle();

  let from_item = menu_handle.get_item(from);
  let to_item = menu_handle.get_item(to);

  to_item.set_enabled(false).unwrap();
  main_window.eval(&format!("window.location.replace('https://www.bing.com/{}')", page_links[to])).unwrap();

  from_item.set_selected(false).unwrap();
  to_item.set_selected(true).unwrap();
  from_item.set_enabled(true).unwrap();
}

fn main() {
  let chat = CustomMenuItem::new("chat".to_string(), "Chat").selected().disabled();
  let image = CustomMenuItem::new("image".to_string(), "Image Generator");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

  let submenu = Submenu::new("File", Menu::new().add_item(quit));
  let window_submenu = Submenu::new("Window",
    Menu::new()
      .add_item(chat)
      .add_item(image)
    );
  
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu)
    .add_submenu(window_submenu);

  tauri::Builder::default()
    .setup(|_| {
      Ok(())
    })
    .menu(menu)
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "chat" => {
          switch_content(event, "image", "chat");
        }
        "image" => {
          switch_content(event, "chat", "image");
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
