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

/// Escapes each item and joins them with unescaped commas.
pub(crate) fn join_text_list<S: AsRef<str>>(items: impl IntoIterator<Item = S>) -> String {
    items
        .into_iter()
        .map(|item| escape_text(item.as_ref()))
        .collect::<Vec<_>>()
        .join(",")
}

/// Splits a raw (still escaped) text list on unescaped commas and unescapes each item.
pub(crate) fn split_text_list(raw: &str) -> Vec<String> {
    if raw.is_empty() {
        return Vec::new();
    }
    let mut items = Vec::new();
    let mut start = 0;
    let mut escaped = false;
    for (i, c) in raw.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            ',' => {
                items.push(unescape_text(&raw[start..i]).into_owned());
                start = i + 1;
            }
            _ => {}
        }
    }
    items.push(unescape_text(&raw[start..]).into_owned());
    items
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

    #[test]
    fn text_list_roundtrip() {
        assert_eq!(split_text_list(&join_text_list(CORPUS)), CORPUS);
    }

    #[test]
    fn split_text_list_on_unescaped_commas_only() {
        assert_eq!(
            split_text_list(r"Projector,Room A\, 2nd floor"),
            ["Projector", "Room A, 2nd floor"]
        );
        assert_eq!(split_text_list(r"a\\,b"), [r"a\", "b"]);
        assert_eq!(split_text_list("a,,b"), ["a", "", "b"]);
        assert!(split_text_list("").is_empty());
    }
}
