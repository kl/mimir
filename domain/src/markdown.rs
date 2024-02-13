use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::sync::OnceLock;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub fn convert_to_html(markdown: &str) -> anyhow::Result<String> {
    static SS: OnceLock<SyntaxSet> = OnceLock::new();

    let syntax_set = SS.get_or_init(SyntaxSet::load_defaults_newlines);

    let parser = Parser::new_ext(markdown, Options::ENABLE_FOOTNOTES | Options::ENABLE_TABLES);

    let hl = highlight(parser, syntax_set)?;

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, hl.into_iter());
    Ok(html_output)
}

fn highlight<'a, I>(events: I, syntax_set: &SyntaxSet) -> Result<Vec<Event<'a>>, anyhow::Error>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut in_code_block = false;

    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    let mut syntax = syntax_set.find_syntax_plain_text();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        syntax = syntax_set
                            .find_syntax_by_token(&lang)
                            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
                    }
                    CodeBlockKind::Indented => {
                        syntax = syntax_set.find_syntax_plain_text();
                    }
                }
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                let mut html = "<pre><code>".to_string();

                let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                    syntax,
                    syntax_set,
                    ClassStyle::Spaced,
                );

                for line in LinesWithEndings::from(&to_highlight) {
                    html_generator.parse_html_for_line_which_includes_newline(line)?;
                }
                html.push_str(html_generator.finalize().as_str());

                html.push_str("</code></pre>");

                to_highlight.clear();
                in_code_block = false;
                out_events.push(Event::Html(CowStr::from(html)));
            }
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    out_events.push(Event::Text(t));
                }
            }
            e => {
                out_events.push(e);
            }
        }
    }
    Ok(out_events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::assert_ok;

    #[test]
    fn test_highlight_output() {
        let output = convert_to_html(
            r#"
```rust
let k = "k".to_string();
let a = 12;
```
"#,
        );
        assert_ok!(output, "<pre><code><span class=\"source rust\">\
        <span class=\"storage type rust\">let</span> k <span class=\"keyword operator rust\">=</span> \
        <span class=\"string quoted double rust\"><span class=\"punctuation definition string begin rust\">&quot;</span>k\
        <span class=\"punctuation definition string end rust\">&quot;</span></span>.\
        <span class=\"support function rust\">to_string</span><span class=\"meta group rust\">\
        <span class=\"punctuation section group begin rust\">(</span></span>\
        <span class=\"meta group rust\"><span class=\"punctuation section group end rust\">)</span></span>\
        <span class=\"punctuation terminator rust\">;</span>\n<span class=\"storage type rust\">let</span> a \
        <span class=\"keyword operator rust\">=</span> <span class=\"constant numeric integer decimal rust\">12\
        </span><span class=\"punctuation terminator rust\">;</span>\n</span></code></pre>");
    }
}
