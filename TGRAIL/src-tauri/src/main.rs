#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn log_points(x_pts: Vec<i32>, y_pts: Vec<i32>) {
  println!("{:?}", x_pts);
  println!("{:?}", y_pts);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![log_points])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
