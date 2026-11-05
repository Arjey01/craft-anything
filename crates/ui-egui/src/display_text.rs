//! Presentation of workflow messages and terminal diagnostics, including saved runs.

use crate::i18n::{tr, trf};

/// Remove terminal controls that have no meaning in an egui label. Keep ordinary Unicode.
pub fn plain(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        let control = if c == '\u{1b}' {
            chars.next()
        } else if c == '\u{9b}' {
            Some('[')
        } else if c == '\u{9d}' {
            Some(']')
        } else {
            None
        };
        match control {
            Some('[') => {
                for c in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&c) {
                        break;
                    }
                }
            }
            Some(']') => {
                while let Some(c) = chars.next() {
                    if c == '\u{7}' || c == '\u{9c}' {
                        break;
                    }
                    if c == '\u{1b}' && chars.peek() == Some(&'\\') {
                        chars.next();
                        break;
                    }
                }
            }
            Some(_) => {}
            None if !c.is_control() || matches!(c, '\n' | '\t') => out.push(c),
            None => {}
        }
    }
    out
}

const PROGRESS: &[&str] = &[
    "Error: VipsImage: memory area too small -- should be {required} bytes, you passed {actual}",
    "The builder checked its work: about {similarity}% like the original, {held} of {total} claims hold.",
    "The builder checked its work: about {similarity}% like the original.",
    "The builder checked its work: all {checks} checks pass, {held} of {total} claims hold.",
    "The builder checked its work: all {checks} checks pass.",
    "The builder checked its work: {passed} of {checks} checks pass, {held} of {total} claims hold.",
    "The builder checked its work: {passed} of {checks} checks pass.",
    "Error: VipsImage: memory area too small --- should be {required} bytes, you passed {actual}",
    "Opening {url} in a private browser…",
    "{name} is taking the game's own files out…",
    "{name} couldn't finish taking the files out ({why}). Building with what's there.",
    "The builder stopped ({why}). Picking up where it left off…",
    "Couldn't open its menus and pop-ups at {width} px: {error}",
    "Photographing more pages ({current} of {total}): {path}…",
    "Couldn't photograph {path}: {error}",
    "The AI service is busy or its rate limit was reached, and asked to wait {secs} s. Trying again then…",
];

/// Recover placeholder values from a known workflow template without changing their contents.
fn arguments<'a>(mut text: &'a str, mut template: &str) -> Option<Vec<(String, &'a str)>> {
    let mut args = Vec::new();
    while let Some((prefix, rest)) = template.split_once('{') {
        text = text.strip_prefix(prefix)?;
        let (name, tail) = rest.split_once('}')?;
        let separator = tail.split('{').next()?;
        let end = if separator.is_empty() { text.len() } else { text.find(separator)? };
        args.push((name.to_string(), text.get(..end)?));
        text = text.get(end..)?;
        template = tail;
    }
    (text == template).then_some(args)
}

/// Translate app-owned workflow text without rewriting agent answers or user content.
pub fn progress(text: &str) -> String {
    let text = plain(text);
    let translated = tr(&text);
    if translated != text {
        return translated;
    }
    for template in PROGRESS {
        if let Some(args) = arguments(&text, template) {
            let args: Vec<_> = args.iter().map(|(key, value)| (key.as_str(), *value)).collect();
            return trf(template, &args);
        }
    }
    text
}

const FREE_LIMIT: &str = "Free usage limit reached. Connect a provider or subscribe to OpenCode Go to keep going, or switch models.";

/// Localize known process wrappers and preserve unrecognized vendor output.
fn diagnostic_line(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed == FREE_LIMIT || trimmed == format!("Error: {FREE_LIMIT}") {
        return tr(FREE_LIMIT);
    }
    if let Some((name, rest)) = line.split_once(" stopped with exit code ") {
        if let Some((code, reason)) = rest.split_once(": ") {
            let reason = if reason == FREE_LIMIT || reason == format!("Error: {FREE_LIMIT}") { tr(FREE_LIMIT) } else { progress(reason) };
            return trf("{name} stopped with exit code {code}: {reason}", &[("name", name), ("code", code), ("reason", &reason)]);
        }
        return trf("{name} stopped with exit code {code}.", &[("name", name), ("code", rest.trim_end_matches('.'))]);
    }
    if let Some(name) = line.strip_prefix("What ").and_then(|s| s.strip_suffix(" printed last:")) {
        return trf("What {name} printed last:", &[("name", name)]);
    }
    if let Some(name) = line.strip_suffix(" printed nothing.") {
        return trf("{name} printed nothing.", &[("name", name)]);
    }
    progress(line)
}

/// Localize known diagnostics, preserving unknown vendor output and executable commands.
pub fn diagnostic(text: &str) -> String {
    let text = plain(text);
    let mut command = false;
    text.split('\n')
        .map(|line| {
            if command {
                return line.to_string();
            }
            if line == "To try it yourself, run:" {
                command = true;
            }
            diagnostic_line(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_sequences_do_not_leak_into_labels() {
        assert_eq!(plain("\x1b[91m\x1b[1mError:\x1b[0m ação 日本語"), "Error: ação 日本語");
        assert_eq!(plain("\x1b]8;;https://example.com\x07link\x1b]8;;\x1b\\\nnext"), "link\nnext");
        assert_eq!(plain("text\x1b[91"), "text");
        assert_eq!(plain("\u{9b}31mtest\u{9b}0m"), "test");
    }

    #[test]
    fn formatted_progress_keeps_urls_and_paths() {
        assert_eq!(
            arguments("Opening https://example.com in a private browser…", "Opening {url} in a private browser…"),
            Some(vec![("url".into(), "https://example.com")])
        );
        assert!(arguments("Unknown text", "Opening {url} in a private browser…").is_none());
    }
}
