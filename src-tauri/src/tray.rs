use tauri::{
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
    App
};


pub fn setup(app: &App) -> tauri::Result<()> {
// メニューの追加
let hello_i = MenuItem::with_id(app, "hello", "こんちわ", true, None::<&str>)?;
let quit_i = MenuItem::with_id(app, "quit", "閉じる", true, None::<&str>)?;
let menu = Menu::with_items(app, &[&hello_i, &quit_i])?;
let _tray = TrayIconBuilder::new()
    .menu(&menu)
    .on_menu_event(|app, event| match event.id.as_ref() {
        "hello" => {
            println!("hello menu item was clicked");
        }
        "quit" => {
            println!("quit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    })
    .icon(app.default_window_icon().unwrap().clone())
    .build(app)?;

    Ok(())
}
