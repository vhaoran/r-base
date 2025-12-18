/*
markup = ["_", "*", "[", "]", "(", ")", "~", "`", ">", "#", "+", "-", "=", "|", "{", "}", ".", "!"];
for (var i = 0; i < length; i++) {
            var char = str[i];
            result += (markup.indexOf(char) !== -1) ? ("\\" + char) : char;
        }
       r
*/

pub fn markdown_v2_escape<T>(s: T) -> String
where
    T: AsRef<str> + std::fmt::Display,
{
    let markup = vec![
        "_", "*", "[", "]", "(", ")", "~", "`", ">", "#", "+", "-", "=", "|", "{", "}", ".", "!",
    ];
    let mut s = s.to_string();
    for c in markup {
        if s.contains(c) {
            let sub = format!("\\{c}");
            s = s.replace(c, sub.as_str());
        }
    }
    s
}

pub fn md_escape<T:AsRef<str>>(s: T) -> String {
    self::markdown_v_escape(s.as_ref())
}

pub fn markdown_v_escape(s: &str) -> String {
    // let markup = vec![
    //     "\\", //	backslash
    //     "`",  //	backtick (see also escaping backticks in code)
    //     "*",  //	asterisk
    //     "_",  //	underscore
    //     "{", "}", //	curly braces
    //     "[", "]", //	brackets
    //     "(", ")", //	parentheses
    //     "#", //	pound sign
    //     "+", //	plus sign
    //     "-", //	minus sign (hyphen)
    //     ".", //	dot
    //     "!", //	exclamation mark
    //     "|", //	pipe (se
    // ];
    let markup = vec!["*", "\\", "_", "`", "["];
    let mut s = s.to_string();
    for c in markup {
        if s.contains(c) {
            let sub = format!("\\{c}");
            s = s.replace(c, sub.as_str());
        }
    }
    s
}

#[test]
fn a_1() {
    //---------------------
    let s = "abc";
    println!(
        "-----old: {s}\n------new: {}-----------",
        markdown_v2_escape(s)
    );

    let s = "abc[]()";
    println!(
        "-----old: {s}\n------new: {}-----------",
        markdown_v2_escape(s)
    );
}

#[test]
fn a_3() {
    //---------------------
    let s = "abc";
    println!(
        "-----old: {s}\n----new: {}-----------",
        markdown_v_escape(s)
    );

    let s = "abc[]()";
    println!(
        "-----old: {s}\n------new: {}-----------",
        markdown_v_escape(s)
    );
}
