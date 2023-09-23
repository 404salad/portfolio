use yew::prelude::*;
use crate::site;


#[function_component]
pub fn Footer() -> Html {
    let linklist = vec![
        site {
            title: "leetcode".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        site {
            title: "euler".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        site {
            title: "github".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        site {
            title: "kaggle".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    let links = linklist.iter().map(|site| html! {
    <a href={site.url.clone()}>{format!("{}",site.title)}</a>
}).collect::<Html>();


    html! { 
    <>
        <div class="absolute bottom-24 w-full">
            <div class="flex flex-row justify-around w-full uppercase text-5xl ">
                { links }
            </div>
        </div>
    </>
    }
}


