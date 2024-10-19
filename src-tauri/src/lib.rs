use rand::Rng;
use std::thread;
use std::time::Duration;
use enigo::*;

#[tauri::command]
fn quack(input: String, trans: i32) -> String {
    let input = input.to_lowercase();
    let tw: String = String::from("qa ");
    let te: String = String::from("qak ");
    let fr: String = String::from("qouc ");
    let fv: String = String::from("quack ");
    let sx: String = String::from("qoauck ");
    let mut rnd = 0;
    let length = &input.len();
    let mut output = String::new();
    
    while &output.len() < length {
        rnd = rand::thread_rng().gen_range(0..=4);
        match rnd {
            0 => output.push_str(&tw),
            1 => output.push_str(&te),
            2 => output.push_str(&fr),
            3 => output.push_str(&fv),
            _ => output.push_str(&sx),
        }
    }

    match trans {
        0 => format!("{}\n-# {}", output, input),
        1 => format!("{}\n{}", output, input),
        2 => format!("**{}**\n{}", output, input),
        3 => output,
        _ => format!("Error: Invalid translation type"),
    }
}
#[tauri::command]
fn enigo(input: String) {
    thread::sleep(Duration::from_secs(2));
    println!("{}", &input);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.text(&input);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![quack, enigo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}