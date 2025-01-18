// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_autostart::MacosLauncher;

// 导入系统托盘所需的依赖, 导入的全都是
use tauri::{
    tray::{
        TrayIconBuilder,
        MouseButtonState,
        MouseButton,
        TrayIconEvent
    },
    menu::{
        Menu,
        MenuItem
    },
    Manager
};

mod event_handler;
// mod tray_tool;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            event_handler::close_handler(app.handle());
            let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            // 创建系统托盘
            let _tray = TrayIconBuilder::new()
                // 添加托盘图标
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("全盘搜工具")
                // 添加菜单
                .menu(&menu)
                // 禁用鼠标左键点击图标显示托盘菜单
                .show_menu_on_left_click(false)
                // 监听托盘图标发出的鼠标事件
                .on_tray_icon_event(|tray, event| match event {
                    // 左键点击托盘图标显示窗口
                    TrayIconEvent::Click {
                        id: _,
                        position: _,
                        rect: _,
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                    } => {
                        let win = tray
                            .app_handle()
                            .get_webview_window("main")
                            .expect("REASON");
                        match win.is_visible() {
                            Ok(visible) if !visible => {
                                win.show().unwrap();
                            }
                            Err(e) => eprintln!("{}", e),
                            _ => (),
                        };
                        // 获取窗口焦点
                        win.set_focus().unwrap();
                    }
                    _ => {}
                })
                // 监听菜单事件
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        let win = app.get_webview_window("main").unwrap();
                        match win.is_visible() {
                            Ok(visible) if !visible => {
                                win.show().unwrap();
                            }
                            Err(e) => eprintln!("{}", e),
                            _ => (),
                        };
                        // 获取窗口焦点
                        win.set_focus().unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
