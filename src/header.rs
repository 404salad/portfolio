use yew::prelude::*;
use crate::site;

#[function_component]
pub fn Header() -> Html {
    let linklist = vec![
        site {
            title: "sahil".to_string(),
            url: "https://leetcode.com/404salad/".to_string(),
        },
        site {
            title: "github".to_string(),
            url: "https://github.com/404salad".to_string(),
        },
        site {
            title: "euler".to_string(),
            url: "https://projecteuler.net/profile/404salad.png".to_string(),
        },
        site {
            title: "kaggle".to_string(),
            url: "https://www.kaggle.com/a404salad".to_string(),
        },
        site {
            title: "linkedin".to_string(),
            url: "https://www.linkedin.com/in/sahil-upasane-3a608b276/".to_string(),
        },
    ];

    let links = linklist.iter().map(|site| html! {
    <a class="mx-auto" href={site.url.clone()} target="_blank">
                <img class="mx-auto px-12 py-12" src={format!("assets/{}.svg",site.title.clone())} alt={site.title.clone()}/>
    </a>
    }).collect::<Html>();

    html! { 
        <>
            <div class="w-full flex flex-row flex-wrap gap-32 h-screen">
                {links}
            </div>
        </>
    }
}




