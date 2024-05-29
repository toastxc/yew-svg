use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}
#[function_component]
fn App() -> Html {
    html! {
        <div>
        <svg xmlns="http://www.w3.org/2000/svg" id="svg" width="199" height="199" viewbox ="0 0 199 199">
        <defs>
        <lineargradient id="grad1" x1="0%" x2="0%" y1="100%" y2="0%">
        <stop offset="50" stop-color="#563d7c" />
        </lineargradient>
        </defs>
        <rect x="3" y="3" fill="url(#grad1)" rx="0" ry="0" width="199" height="199" />
        </svg>
        </div>
    }
}
