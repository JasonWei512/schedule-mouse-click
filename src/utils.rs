pub fn get_current_executable_name() -> String {
    if let Some(executable_name) = try_get_current_executable_name() {
        return executable_name;
    }

    "schedule-mouse-click".to_string()
}

fn try_get_current_executable_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}
