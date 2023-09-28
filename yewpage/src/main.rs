use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <div id="header">
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
