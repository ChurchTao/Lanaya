use base64::engine::general_purpose;
use base64::Engine;
use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(s);
    hasher.result_str()
}

#[allow(unused)]
pub fn md5_by_bytes(bytes: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.input(bytes);
    hasher.result_str()
}

pub fn base64_encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

pub fn base64_decode(base64: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64).unwrap()
}

// 如果content中 包含key的话，就把key用<b>key</b>高亮起来
pub fn highlight(key: &str, content: &str) -> String {
    let mut res = String::new();
    let mut start = 0;
    let mut end;

    while let Some(i) = content[start..].to_lowercase().find(&key.to_lowercase()) {
        end = start + i;
        res.push_str(&content[start..end]);
        res.push_str(&format!(
            "[highlight]{}[/highlight]",
            &content[end..end + key.len()]
        ));
        start = end + key.len();
    }
    res.push_str(&content[start..]);
    res = escape_html(&res);
    res = res
        .replace("[highlight]", "<b>")
        .replace("[/highlight]", "</b>");
    res
}

fn escape_html(html: &str) -> String {
    html.replace("<", "&lt;").replace(">", "&gt;")
}

#[test]
fn test_highlight() {
    let res = highlight("hello", "hello worldhello");
    println!("{}", res);
}
