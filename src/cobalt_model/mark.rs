use std::sync::LazyLock;

use clap::builder::Str;
use log::info;
use pulldown_cmark::{self as cmark, CowStr};
use regex::{Regex, RegexBuilder};
use serde::Serialize;

use crate::error::Result;
use crate::syntax_highlight::decorate_markdown;

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MarkdownBuilder {
    pub base_url: Option<String>,
    pub theme: Option<liquid::model::KString>,
    #[serde(skip)]
    pub syntax: std::sync::Arc<crate::SyntaxHighlight>,
}

impl MarkdownBuilder {
    pub fn build(self) -> Markdown {
        Markdown {
            base_url: self.base_url,
            theme: self.theme,
            syntax: self.syntax,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Markdown {
    base_url: Option<String>,
    theme: Option<liquid::model::KString>,
    syntax: std::sync::Arc<crate::SyntaxHighlight>,
}

impl Markdown {
    pub fn parse(&self, content: &str, url_path: &str) -> Result<String> {
        let mut buf = String::new();
        let options = cmark::Options::ENABLE_FOOTNOTES
            | cmark::Options::ENABLE_TABLES
            | cmark::Options::ENABLE_STRIKETHROUGH
            | cmark::Options::ENABLE_TASKLISTS;
        let parser = cmark::Parser::new_ext(content, options);
        let with_highlights =
            decorate_markdown(parser, self.syntax.clone(), self.theme.as_deref())?;
        let with_resolved_links = XLink::new(with_highlights, self.base_url.as_deref(), url_path);
        cmark::html::push_html(&mut buf, with_resolved_links);
        Ok(buf)
    }
}

struct XLink<'a, Inner: Iterator<Item = cmark::Event<'a>>> {
    /// Inner iterator around commonmark events
    inner: Inner,
    base_url: Option<&'a str>,
    url_path: &'a str,

    /// Full URL of the markdown document. Using https://cobalt.example.com as base if unset.
    document_url: url::Url,
}

static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new("^(?:[a-z][a-z0-9+\\-.]*:)?//")
        .case_insensitive(true)
        .build()
        .expect("valid regex")
});

fn make_absolute_url(base: Option<&str>, link: &str) -> url::Url {
    let base = base.unwrap_or("https://cobalt.example.com");
    let base = url::Url::parse(base).unwrap_or_else(|_| {
        url::Url::parse("https://cobalt.example.com").expect("valid hardcoded url")
    });
    base.join(link).unwrap_or_else(|_| {
        url::Url::parse("https://cobalt.example.com").expect("valid hardcoded url")
    })
}

impl<'a, Inner: Iterator<Item = cmark::Event<'a>>> XLink<'a, Inner> {
    fn new(inner: Inner, base_url: Option<&'a str>, url_path: &'a str) -> Self {
        Self {
            inner,
            base_url,
            url_path,
            document_url: make_absolute_url(base_url, url_path),
        }
    }

    fn normalize_link(&self, link: CowStr<'a>) -> CowStr<'a> {
        let base_url = match self.base_url {
            Some(b) => b,
            None => {
                info!("No base_url set, not normalizing link: {}", link);
                return link
            },
        };

        if link.starts_with("/") || link.starts_with('#') || URL_REGEX.is_match(&link) {
            info!("Not normalizing link: {} with url_path: {} and base_url {}, document_url {}", link, self.url_path, base_url, &self.document_url);
            return link;
        }

        info!(
            "Normalizing link: {} with url_path: {} and base_url {}, document_url {}",
            link, self.url_path, base_url, &self
            .document_url
        );
        let base = url::Url::parse(base_url).expect("valid base url");

        base.join(self.url_path)
            .and_then(|u| u.join(&link))
            .expect("valid join")
            .to_string()
            .into()
    }
}

impl<'a, Inner: Iterator<Item = cmark::Event<'a>>> Iterator for XLink<'a, Inner> {
    type Item = cmark::Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(cmark::Event::Start(cmark::Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            })) => {
                let dest_url = self.normalize_link(dest_url);

                Some(cmark::Event::Start(cmark::Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                }))
            }
            Some(cmark::Event::Start(cmark::Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            })) => {
                let dest_url = self.normalize_link(dest_url);

                Some(cmark::Event::Start(cmark::Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }))
            }
            event => event,
        }
    }
}
