// 导入系统托盘所需的依赖, 导入的全都是
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
    AppHandle,
    Wry
};


pub fn create_tray(app: &AppHandle<Wry>) -> std::io::Result<()> {
    let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
    // 创建系统托盘
    let _tray = TrayIconBuilder::new()
        // 添加托盘图标
        .icon(app.default_window_icon().unwrap().clone())
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
}
