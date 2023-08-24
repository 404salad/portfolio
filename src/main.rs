mod app;
use app::App;
mod header;use header::Header;

fn main() {
    yew::Renderer::<Header>::new().render();
    yew::Renderer::<App>::new().render();
}
