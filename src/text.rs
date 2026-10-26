//! Escaping of [TEXT](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11) values.

use std::borrow::Cow;


pub(crate) fn escape_text(input: &str) -> String {
    input
        .replace('\\', r#"\\"#)
        .replace(',', r#"\,"#)
        .replace(';', r#"\;"#)
        .replace('\n', r#"\n"#)
}

/// Reverses [`escape_text`].
///
/// Single left-to-right pass so it is the exact inverse of the escaper:
/// chained `String::replace` collapsed `\\` first, freeing the backslash
/// to recombine with the next char and be eaten by a later rule.
#[cfg_attr(not(feature = "parser"), allow(dead_code))]
pub(crate) fn unescape_text(input: &str) -> Cow<'_, str> {
    if !input.contains('\\') {
        return Cow::Borrowed(input);
    }
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some(',') => out.push(','),
            Some(';') => out.push(';'),
            // lenient: some producers escape the (unreserved) colon
            Some(':') => out.push(':'),
            Some('n') | Some('N') => out.push('\n'),
            // unknown escape or trailing backslash: keep verbatim
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    Cow::Owned(out)
}


#[cfg(test)]
mod tests {
    use super::*;

    const CORPUS: [&str; 12] = [
        "plain text",
        "Hello, world; test:value",
        r"a\nb",         // literal backslash-n, not a newline
        r"a\Nb",         // literal backslash-N, not a newline
        r"a\:b",         // literal backslash before a colon
        r"C:\next\node", // Windows path
        r"back\\slash",
        "comma,semicolon;colon:end",
        "newline\nhere",
        "trailing backslash\\",
        r"mix \, \; \\ \n done",
        "üni¢ode ☃ \\ , ; \n",
    ];

    #[test]
    fn escape_special_characters() {
        assert_eq!(escape_text("\n\\;,:"), r"\n\\\;\,:");
    }

    #[test]
    fn unescape_is_inverse_of_escape() {
        for original in CORPUS {
            assert_eq!(unescape_text(&escape_text(original)), original);
        }
    }

    #[test]
    fn unescape_is_lenient() {
        // `\N` is the RFC-5545 newline escape; `\:` and unknown escapes are
        // non-standard producer output we have always accepted.
        assert_eq!(unescape_text(r"a\:b"), "a:b");
        assert_eq!(unescape_text(r"line\Nbreak"), "line\nbreak");
        assert_eq!(unescape_text(r"tab\tend"), r"tab\tend");
        assert_eq!(unescape_text("trailing\\"), "trailing\\");
    }

    #[test]
    fn unescape_borrows_when_nothing_to_do() {
        assert!(matches!(unescape_text("plain"), Cow::Borrowed(_)));
    }

}
