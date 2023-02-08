use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(s);
    hasher.result_str()
}

// 如果content中 包含key的话，就把key用<b>key</b>高亮起来
pub fn highlight(key: &str, content: &str) -> String {
    let mut res = String::new();
    let mut start = 0;
    let mut end;

    while let Some(i) = content[start..].find(key) {
        end = start + i;
        res.push_str(&content[start..end]);
        res.push_str(&format!("<b>{}</b>", &content[end..end + key.len()]));
        start = end + key.len();
    }
    res.push_str(&content[start..]);
    res
}

#[test]
fn test_highlight() {
    let res = highlight("hello", "hello worldhello");
    println!("{}", res);
}
