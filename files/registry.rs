//! Windows registry read helpers for Graphisoft Bimx.

pub const APP_KEY: &str = "Software\\Graphisoft\\Graphisoft Bimx";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Graphisoft Bimx";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Graphisoft Bimx") },
        RegValue { name: "Version",     value: String::from("v4.9.8") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Graphisoft Bimx") },
        RegValue { name: "Publisher",       value: String::from("Graphisoft") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Graphisoft Bimx\\uninstall.exe\"") },
    ]
}
