pub fn get(package_manager: &str, script_name: &str, args: Option<&str>) -> String {
    if script_name == "install" {
        return if package_manager == "yarn" {
            "yarn".to_string()
        } else {
            format!("{} install", package_manager)
        };
    }

    if let Some(args) = args {
        return if package_manager == "npm" {
            format!("npm run {} -- {}", script_name, args)
        } else {
            format!("{} {} {}", package_manager, script_name, args)
        };
    } else {
        return if package_manager == "npm" {
            format!("npm run {}", script_name)
        } else {
            format!("{} {}", package_manager, script_name)
        };
    }
}
