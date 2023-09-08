mod app; use app::App;
mod header; use header::Header;
mod footer; use footer::Footer;

fn main() {
    yew::Renderer::<Header>::new().render();
    yew::Renderer::<App>::new().render();
    yew::Renderer::<Footer>::new().render();
}
