// Форматирование времени для отображения
use chrono::Local;

pub fn get_formatted_time(show_colon: bool) -> String {
    let now = Local::now();
    if show_colon {
        now.format("%H:%M").to_string()
    } else {
        now.format("%H %M").to_string()
    }
}