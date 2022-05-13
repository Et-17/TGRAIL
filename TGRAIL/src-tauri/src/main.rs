#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::char_rec::Point;

mod char_rec;

#[tauri::command]
fn log_points(x_pts: Vec<i32>, y_pts: Vec<i32>) -> Vec<Point> {
    let zipped = char_rec::zip_x_y_path(x_pts, y_pts);
    let corners = char_rec::find_corners(zipped, 22.5, 90.0);
    println!("");
    corners.iter().for_each(|x| println!("{:?}", x));
    corners
    //println!("{:?}", char_rec::find_corners(zipped, 16));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log_points])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
