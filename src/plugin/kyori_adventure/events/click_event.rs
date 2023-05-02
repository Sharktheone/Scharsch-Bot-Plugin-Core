use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickEvent {
    action: String,
    run_command: Option<String>,
    suggest_command: Option<String>,
    open_url: Option<String>,
    copy_to_clipboard: Option<String>,
    open_file: Option<String>,
    change_page: Option<i32>,
    change_page_str: Option<String>,
}