use yew::prelude::*;
#[function_component]
pub fn Footer() -> Html {
    html! {
        <div class="border border-black md:border-red sm:border-white">
            <p class="bg-red-400 ">{"this a footer"}</p>
        </div>

    }
}

