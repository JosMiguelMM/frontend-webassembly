use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    nombre: String,
}
//crear un nuevo componente
#[function_component]
fn VideoControls(props: &VideoSectionProps) -> Html {
    let id: String = format!("https://www.youtube.com/embed/{}", props.id);
    let nombre = props.nombre.clone();
    //zvsLQ2ejJOw
    html! {
        <dir>
            <h1 class={classes!("title")}>{nombre}</h1>
            <iframe class={classes!("video")}  src={id} frameborder="0"></iframe>
        </dir>
    }
}

//nueva struct
#[derive(Properties, PartialEq)]
struct VideoControlsPorps {
    on_search: Callback<String>,
}

#[function_component]
fn Controls(props: &VideoControlsPorps) -> Html {
    html! {
       <div class={classes!("control")}>
        <span class={classes!("mensaje")}>{"Ingrese una palabra"}</span>
        <input type="text" class={classes!("input")} placeholder="Ingrese una palabra"/>
        <button class={classes!("button")}>{"Buscar"}</button>
       </div>
    }
}

struct Video {
    id: String,
    nombre: String,
}

#[function_component]
fn App() -> Html {
    let video: UseStateHandle<Option<Video>> = use_state(|| None);
    let on_search: Callback<String> = Callback::from(|_| {});
    html! {

        <main class={classes!("container")} style=" text-align: center; ">
            <Controls on_search={on_search}/>
            <VideoControls nombre="Sistema resuelto" id="zvsLQ2ejJOw"/>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
