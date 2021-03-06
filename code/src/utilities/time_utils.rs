use chrono::{Datelike, Utc};

pub fn handle_time() -> String {
  let now = Utc::now();
  return format!("{}-{}-{}", now.year(), now.month(), now.day());
}
