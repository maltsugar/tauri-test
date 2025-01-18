use tauri::{AppHandle, Emitter, Manager, Wry};
use tauri::WindowEvent;



pub fn close_handler(app: &AppHandle<Wry>) {
    let main_window = app.get_webview_window("main").unwrap();
    let main_window_clone = main_window.clone();
    let app_clone = app.clone();
    main_window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            // print!("点击关闭");
            // 阻止默认的关闭行为
            api.prevent_close();
            main_window_clone.hide().unwrap();
            app_clone.emit("didClickClose", 1).unwrap();
        }
    });
}
