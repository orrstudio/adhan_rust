// SQLite база данных — настройки, окна, молитвы
// TODO: реализовать SettingsDatabase на rusqlite

pub struct SettingsDatabase;

impl SettingsDatabase {
    pub fn new() -> Self {
        Self
    }

    pub fn get_setting(&self, _key: &str) -> Option<String> {
        None
    }

    pub fn save_setting(&self, _key: &str, _value: &str) {
        // заглушка
    }
}
