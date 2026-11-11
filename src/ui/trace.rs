use dioxus::prelude::*;

pub async fn sample_glyph(text: &str) -> Vec<(f32, f32)> {
    let payload = serde_json::to_string(text).unwrap_or_else(|_| "\"\"".to_string());
    let script = format!(
        r##"const text = {payload};
await document.fonts.ready;
const canvas = document.createElement("canvas");
canvas.width = 400;
canvas.height = 300;
const ctx = canvas.getContext("2d");
const size = text.length > 3 ? 78 : text.length > 1 ? 120 : 170;
ctx.font = size + "px Saksham Devanagari, Nirmala UI, sans-serif";
ctx.fillStyle = "#000";
ctx.textAlign = "center";
ctx.textBaseline = "middle";
ctx.fillText(text, 200, 155);
const img = ctx.getImageData(0, 0, 400, 300).data;
const dots = [];
const step = text.length > 2 ? 8 : 11;
for (let y = 0; y < 300; y += step) {{
  for (let x = 0; x < 400; x += step) {{
    if (img[(y * 400 + x) * 4 + 3] > 90) dots.push([x / 400 * 100, y / 300 * 100]);
  }}
}}
return dots;"##
    );
    match document::eval(&script).await {
        Ok(value) => serde_json::from_value(value).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
