use rand::Rng;

#[tauri::command]
fn quack(input: String) -> String {
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
        rnd = rand::thread_rng().gen_range(1..=4);
        if rnd == 0 {
            output.push_str(&tw);
        } else if rnd == 1 {
            output.push_str(&te);
        } else if rnd == 2 {
            output.push_str(&fr);
        } else if rnd == 3 {
            output.push_str(&fv);
        } else {
            output.push_str(&sx);
        }
    }
    output
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![quack])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
