use super::*;

pub(crate) async fn home(
    Extension(_config): Extension<AstelConfig>,
    Extension(html): Extension<HtmlContextBuilder>,
) -> impl IntoResponse {
    let content = html! {
        h1 {
            "Home"
        }

        p {
            "This dashboard is empty. You can customize it with "
            code {
                "Astel::main_dashboard"
            }
            "."
        }
    };

    html.build(content).with_title("Astel - Home")
}
