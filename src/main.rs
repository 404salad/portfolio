mod footer; use footer::Footer;
mod header; use header::Header;

pub struct site {
    title: String,
    url: String,
}

fn main() {
    yew::Renderer::<Header>::new().render();
    yew::Renderer::<Footer>::new().render();
}
