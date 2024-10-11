mod structs;
mod util;
mod xzmu;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            xzmu::test_network,
            xzmu::get_conf,
            xzmu::get_account,
            xzmu::save_account,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
