//! Module to process WordStar dot commands

// Utilises new "bool then" feature in Rust 1.50 to simplify use of '?' operator
//     [condition].then(|| ())
//  -> if [condition] { Some( () ) } else { None }
//
fn check_dot_cmd(s: &str) -> Option<(&str, Option<&str>)> {
    let go_on = || ();
    let is_dot = |&c: &char| c == '.';

    let mut iter = s.char_indices();
    let (_, c) = iter.next()?;
    is_dot(&c).then(go_on)?;
    let (i, c) = iter.next()?;
    char::is_ascii_alphabetic(&c).then(go_on)?;
    let (_, c) = iter.next()?;
    char::is_ascii_alphanumeric(&c).then(go_on)?;
    match iter.next() {
        Some((j, _)) => Some((&s[i..j], Some(&s[j..]))),
        None => Some((&s[i..], None)),
    }
}

fn strip_control_chars(s: &str) -> String {
    s.chars()
        .filter(|c| !char::is_ascii_control(c))
        .collect::<String>()
}

fn make_header(prefix: &str, opt_text: Option<&str>) -> Option<String> {
    let text = opt_text?;
    let mut result = String::from(prefix);
    let conv_text = strip_control_chars(text);
    result.push_str(conv_text.trim());
    Some(result)
}

pub fn process_dot_cmd(line: &str) -> Option<String> {
    let (cmd, opt_text) = check_dot_cmd(line)?;
    let lower_cmd = cmd.to_ascii_lowercase();
    match &lower_cmd[..] {
        "he" | "fo" => make_header("## ", opt_text),
        "h1" | "h2" | "h3" | "h4" | "h5" | "f1" | "f2" | "f3" | "f4" | "f5" => {
            make_header("### ", opt_text)
        }
        "pa" | "xl" => Some("---".to_string()),
        _ => Some("".to_string()),
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dot_cmds() {
        assert_eq!(check_dot_cmd(".cw 8"), Some(("cw", Some(" 8"))));
        assert_eq!(check_dot_cmd(".op"), Some(("op", None)));
        assert_eq!(check_dot_cmd(".h4"), Some(("h4", None)));
        assert_eq!(check_dot_cmd(".4h"), None);
        assert_eq!(check_dot_cmd(".h!"), None);
        assert_eq!(check_dot_cmd("abc"), None);
        assert_eq!(check_dot_cmd(""), None);
    }

    #[test]
    fn test_strip_control_chars() {
        let text = "\x08  jdj  \x06df  kf\x08\x08\x08  ";
        assert_eq!(strip_control_chars(text), "  jdj  df  kf  ");
        assert_eq!(strip_control_chars("abc"), "abc");
        assert_eq!(strip_control_chars("\x08\x13"), "");
        assert_eq!(strip_control_chars(""), "");
    }

    #[test]
    fn test_make_header() {
        assert_eq!(
            make_header("# ", Some("hello")),
            Some("# hello".to_string())
        );
        assert_eq!(
            make_header("# ", Some("he\x03llo")),
            Some("# hello".to_string())
        );
        assert_eq!(make_header("# ", None), None);
    }

    #[test]
    fn test_process_dot_cmd() {
        let text = ".He \x03 jd \x04 jhhfjf*¬£   \x05  ";
        assert_eq!(process_dot_cmd(text), Some("## jd  jhhfjf*¬£".to_string()));
        assert_eq!(
            process_dot_cmd(".f3 \x13\x14TEST\x13\x14"),
            Some("### TEST".to_string())
        );
        assert_eq!(process_dot_cmd(".op"), Some("".to_string()));
        assert_eq!(process_dot_cmd("abc"), None);
        assert_eq!(process_dot_cmd(""), None);
    }
}
