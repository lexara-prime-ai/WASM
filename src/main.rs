use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h2 class={ "header" }>
                { "Stream " }
                <span class={ "highlighted-bg" }>
                    { "Tracker" }
                </span>
            </h2>

            <section class={ "content" }>
                <div class={ "streamer" }>
                    <img class={ "banner" } src={ "/images/gigguk.jpg" } alt="banner"/>

                    <div class={ "streamer-info" }>
                        <h2 class={ "stream-title" }>{ "Gigguk | Summer anime tier list stream..." }</h2>
                        <p class={ "stream-description" }>
                            { "This is a short description about the what the stream is about feel free to..." }
                        </p>
                    </div>
                </div>

                <div class={ "streamer" }>
                    <img class={ "banner" } src={ "/images/moistcritikal.webp" } alt="banner"/>

                    <div class={ "streamer-info" }>
                        <h2 class={ "stream-title" }>{ "Moist Critikal | Worst horror game stream..." }</h2>
                        <p class={ "stream-description" }>
                            { "This is a short description about the what the stream is about feel free to..." }
                        </p>
                    </div>
                </div>

                <div class={ "streamer" }>
                    <img class={ "banner" } src={ "/images/timhenson.webp" } alt="banner"/>

                    <div class={ "streamer-info" }>
                        <h2 class={ "stream-title" }>{ "Tim Henson | Home Studio showcase..." }</h2>
                        <p class={ "stream-description" }>
                            { "This is a short description about the what the stream is about feel free to..." }
                        </p>
                    </div>
                </div>

            </section>
        </div>
    }
}

fn main() {
    println!("Doing something");
    yew::Renderer::<App>::new().render();
}


