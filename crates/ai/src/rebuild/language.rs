//! Language guidance for user-facing agent responses; machine identifiers stay unchanged.

/// Keep replies in the selected Portuguese locale without changing the person's task.
pub(super) fn localized_prompt(text: &str, language: &str) -> String {
    let tag = language.trim().split(['.', '@']).next().unwrap_or_default().replace('_', "-").to_ascii_lowercase();
    if tag.split('-').next() == Some("pt") {
        format!(
            "Respond to the person in natural Brazilian Portuguese (pt-BR), including progress updates, explanations, checklist labels and questions. Preserve code, commands, paths and technical identifiers. Follow an explicit request for another response language when the person makes one. This language preference does not change the requested language or content of the software you create.\n\n{text}"
        )
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portuguese_response_guidance_preserves_the_task_and_technical_identifiers() {
        let task = "Create an English website in app/ with Rust/macroquad; run cargo test.";
        for tag in ["pt", "pt-br", "PT_BR.UTF-8", "pt-PT"] {
            let prompt = localized_prompt(task, tag);
            assert!(prompt.contains("natural Brazilian Portuguese (pt-BR)"));
            assert!(prompt.contains("does not change the requested language or content"));
            assert!(prompt.ends_with(task));
        }
        for tag in ["en", "ja", "", "ptx"] {
            assert_eq!(localized_prompt(task, tag), task);
        }
    }
}
