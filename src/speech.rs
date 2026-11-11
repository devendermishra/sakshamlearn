use dioxus::prelude::*;

pub fn speak(text: &str, enabled: bool) {
    if !enabled || text.trim().is_empty() {
        return;
    }
    let payload = serde_json::to_string(text).unwrap_or_else(|_| "\"\"".to_string());
    let script = format!(
        r#"const u = new SpeechSynthesisUtterance({payload});
u.lang = "hi-IN";
u.rate = 0.9;
window.speechSynthesis.cancel();
window.speechSynthesis.speak(u);"#
    );
    spawn(async move {
        let _ = document::eval(&script).await;
    });
}
