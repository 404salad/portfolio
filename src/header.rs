use yew::prelude::*;
#[function_component]
pub fn Header() -> Html {
    html! { 
        <div class="flex justify-between bg-red-400 " id = "header"> 
            <a src="/">{"logo"}</a>
            <a src="/">{"about"}</a>
        </div>
    }
}
