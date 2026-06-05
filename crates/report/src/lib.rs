pub mod markdown;
pub mod json;

pub fn render_all() -> Vec<String> {
    vec![markdown::render(), json::render()]
}
