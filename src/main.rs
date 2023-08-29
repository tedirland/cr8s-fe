use yew::prelude::*;

// this is the root component of the application
// two options - functional components and object component
// FCs are a lot easier to understand, maintain and read
#[function_component(App)]
fn app() -> Html {
    html!{
        <>
        <p>{"Hello world"}</p>

        </>

    }
}



fn main() {
    yew::Renderer::<App>::new()
    .render();
}
