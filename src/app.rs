use yew::prelude::*;

#[function_component(App)]

pub fn app() -> Html {
    let counter = use_state(||0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{*counter}</p>
            <p>{"text in a paragraph"}</p>
            <div class="bg-blue-200">{"text ina div"}</div>
        </div>

    }
}

