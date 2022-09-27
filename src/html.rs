use std::collections::HashMap;

use crate::config::AstelConfig;
use axum::{
    http::Request,
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use maud::{html, Markup, DOCTYPE};

/// gets inserted as an extension into the request by `HtmlMiddleware`
/// use the `build` method to provide it the html content
#[derive(Clone)]
pub(crate) struct HtmlContextBuilder {
    pub(crate) config: AstelConfig,
}

pub async fn html_context_middleware<B: Send>(
    mut req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let config = req.extensions().get::<AstelConfig>().unwrap().clone();

    req.extensions_mut().insert(HtmlContextBuilder { config });

    next.run(req).await
}

impl HtmlContextBuilder {
    pub(crate) fn build(self, content: Markup) -> HtmlContext {
        HtmlContext {
            content,
            config: self.config,

            title: None,
            description: None,

            sections: Default::default(),
        }
    }
}

pub(crate) struct HtmlContext {
    pub content: Markup,
    pub config: AstelConfig,

    pub title: Option<String>,
    pub description: Option<String>,

    pub sections: HashMap<String, Vec<Markup>>,
}

impl HtmlContext {
    /// sets the title for this page
    /// will be `Config::app_name` by default
    pub fn with_title(mut self, s: impl ToString) -> Self {
        self.title = Some(s.to_string());
        self
    }
    pub fn get_title(&self) -> &str {
        self.title.as_deref().unwrap_or("astel")
    }

    /// sets the description for this page
    pub fn with_description(mut self, s: impl ToString) -> Self {
        self.description = Some(s.to_string());
        self
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// add a piece of markdown to a section, similar to laravel's `@push`
    /// usually used for adding `script`s at the bottom of the page
    pub fn section_append(mut self, key: impl ToString, m: Markup) -> Self {
        let section = self.sections.entry(key.to_string()).or_default();
        section.push(m);
        self
    }

    pub fn section_get(&self, key: &str) -> Markup {
        let section: &[Markup] = self
            .sections
            .get(key)
            .map(AsRef::as_ref)
            .unwrap_or_default();
        html! {
            @for i in section {(*i)}
        }
    }
}

impl IntoResponse for HtmlContext {
    fn into_response(self) -> Response {
        let m = html! {
            (DOCTYPE)
            head {
                title {
                    (self.get_title())
                }

                // TODO allow customization
                link rel="stylesheet" href="/astel/css/main.css" type="text/css";
            }
            body {
                .categories {
                    h2 {
                        // TODO change this
                        a href=(&self.config.path) {
                            "Astel"
                        }
                    }

                    @for name in &self.config.names {
                        a href={(&self.config.path)"/"(name)} {
                            (name)
                        }
                    }
                }
                div.container {
                    (self.content)
                }
            }
        };

        Html(m.into_string()).into_response()
    }
}
