use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    nombre: String,
}

//crear un nuevo componente
#[function_component]
fn VideoControls(props:&VideoSectionProps) -> Html {
    let id:String=format!("https://www.youtube.com/embed/{}",props.id);
    let nombre=props.nombre.clone();
    //zvsLQ2ejJOw
    html! {
        <dir>
            <iframe width="600" height="400" src={id} frameborder="0"></iframe>
        </dir>
    }
}

#[function_component]
fn Formulario() -> Html {
    html! {
        <form class={classes!("formulario")}>
            <label for="fname">{"First name:"}</label><br />
            <input type="text" id="fname" name="fname" /><br />
            <label for="lname">{"Last name:"}</label><br />
            <input type="text" id="lname" name="lname" />
        </form>
    }
}

#[function_component]
fn SaludoYSuma() -> Html {
    let nombre: &str = "Jose Miguel";
    html! {
        <div>
            <h1>{nombre}</h1>
        </div>
    }
}

#[function_component]
fn App() -> Html {

    html! {

        <main class={classes!("container")} style=" text-align: center; ">
            <SaludoYSuma/>
            <VideoControls nombre="Sistema resuleto" id="zvsLQ2ejJOw"/>
            <Formulario/>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
