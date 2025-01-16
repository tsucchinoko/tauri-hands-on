use tauri::{App, Result};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg(desktop)]
pub fn setup(app: &App) -> Result<()> {
    let option_m_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyM);

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                if shortcut == &option_m_shortcut {
                    match event.state() {
                        ShortcutState::Pressed => {
                            println!("Option-U Released!");

                        }
                        ShortcutState::Released => {
                            println!("Option-U Released!");
                        }
                    }
                }
            })
            .build(),
    )?;

    app.global_shortcut().register(option_m_shortcut).unwrap();
    Ok(())
}
