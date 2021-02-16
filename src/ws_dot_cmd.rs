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
        Some((j, ' ')) => Some((&s[i..j], Some(&s[j..]))),
        None => Some((&s[i..], None)),
        Some(_) => None,
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

/*
fn main() {
    println!("{} {}", ws_constants::ABC, ws_constants::CH);

    let mut s = String::new();
    s.push('b');
    s.push('a');
    s.push('r');

    if check_str(&s) {
        println!("Success!");
    } else {
        println!("Failure!");
    }
    
    let s = "bjfkf".to_ascii_uppercase();
    println!("{}", s);

    let text = "\x08  jdj  \x06df  kf\x08\x08\x08  ";
    let s = strip_control_chars(text);
    println!("{}", s.trim());

    let text = ".He \x03 jd \x04 jhhfjf*¬£   \x05  ";
    // let text = ".he";
    println!(".he -> {:?}", process_dot_cmd(text));

    let text = ".pA";
    println!(".pa -> {:?}", process_dot_cmd(text));
    
    let text = ".co 4";
    println!(".co -> {:?}", process_dot_cmd(text));
}
*/