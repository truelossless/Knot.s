const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Gets the lv1 title number as a roman thingy
// CAVEAT: ugly and doesn't work after 39
pub fn get_roman_numeral(num: usize) -> String {
    let mut buf = String::new();

    let unit = match num % 10 {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => unreachable!(),
    };

    let dozen_num = num / 10;
    for _ in 0..dozen_num {
        buf.push('X');
    }

    buf += unit;
    buf
}

/// Gets a lv2 title number as a letter
/// CAVEAT: only 26 possibilities
pub fn get_alpha_numeral(mut num: usize) -> String {
    num -= 1;

    if num >= 25 {
        num = 25
    }

    ALPHABET[num..num + 1].to_owned()
}

/// Wkhtmltopdf 0.12 doesn't support template literals 🤣🤣🤣🔫
pub fn escape_latex(input: &str) -> String {
    let mut res = input.replace("\r\n", "\n");
    res = res.replace("\n", r#"\\"#);
    res = res.replace("\\", r#"\\"#);
    res = res.replace("'", r#"\'"#);
    res
}

/// Escapes an HTML string
// While we allow our users to directly write HTML,
// we shouldn't render it in code blocks.
pub fn escape_html(input: &str) -> String {
    let mut res = input.replace("&", "&amp;");
    res = res.replace("<", "&lt;");
    res = res.replace(">", "&gt;");
    res
}
