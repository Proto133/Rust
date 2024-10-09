use yew::prelude::*;

pub struct AppFooter;

impl Component for AppFooter {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let css_content = include_str!("footer.css");
        // Find the start and end of the inline styles in the CSS content
        let start = css_content.find('{').unwrap_or(0) + 1;
        let end = css_content.rfind('}').unwrap_or(css_content.len());
        let inline_styles = css_content[start..end].trim();

        html! {
            <footer style={inline_styles}>
                <p>{ "© 2024 My Website. All rights reserved." }</p>
            </footer>
        }
    }
}
