use std::fs;
const LAST_USER_NAME: &str = "/var/cache/marine_greetdm/last-username";

pub fn get_last_user_name() -> Option<String> {
  match fs::read_to_string(LAST_USER_NAME).ok() {
    None => None,
    Some(name) => {
      let name = name.trim();

      if name.is_empty() {
        None
      } else {
        Some(name.to_string())
      }
    }
  }
}

pub fn write_last_user_name(username: &str) {
    let _ = fs::write(LAST_USER_NAME, username);
}
