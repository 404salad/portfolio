use yew::prelude::*;
use crate::site;


#[function_component]
pub fn Footer() -> Html {
    let linklist = vec![
        site {
            title: "leetcode".to_string(),
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
            url: "https://linkedin.com/sahilupasanwe".to_string(),
        },
    ];

    let links = linklist.iter().map(|site| html! {
    <a class="mx-auto" href={site.url.clone()} target="_blank">{format!("{}",site.title)}</a>
}).collect::<Html>();


    html! { 
    <>
        <div class="absolute bottom-24 w-full">
            <div class="flex flex-col lg:flex-row justify-around w-full gap-32 lg:gap-0 uppercase text-3xl lg:text-6xl">
                { links }
            </div>
        </div>
    </>
    }
}


