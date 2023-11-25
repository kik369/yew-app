use gloo::console;
use gloo_net::http::Request;
use js_sys::Date;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos
        .iter()
        .map(|video| {

            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| {
                    on_click.emit(video.clone())
                })
            };

            html! {
                <p key={video.id} onclick={on_video_select}>{format!("{}: {} is this here?", video.speaker, video.title)}</p>
            }
        })
        .collect()
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <p>{ format!("🎙️ {}", video.speaker.clone()) }</p>
            <img src="https://ik.imagekit.io/refaktor/user.webp" alt="video thumbnail" />
        </div>
    }
}

// Define your application routes
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/post/:id")]
    Post { id: usize },
}

#[derive(Properties, PartialEq)]
struct PostProps {
    id: usize,
}

// Define the About component
#[function_component(About)]
fn about() -> Html {
    html! {
        <div>
            <Nav />
            <h2>{ "About RustConf Explorer" }</h2>
            <p>{ "This is a web application built using Yew for exploring RustConf videos." }</p>
        </div>
    }
}

// Route::Post { id } => html! {<p>{format!("You are looking at Post {}", id)}</p>},
#[function_component(Post)]
fn post(PostProps { id }: &PostProps) -> Html {
    html! {
        <div>
            <Nav />
            <h2>{ "Post" }</h2>
            <p>{ format!("You are looking at Post {}", id) }</p>
        </div>
    }
}

// Modify your App component to include a router
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav>
            <span>{ "🏠 " }</span>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <span>{ " 📝 " }</span>
            <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
            <span>{ " 🪶 " }</span>
            <Link<Route> to={Route::Post { id: 3 }}>{ "Post 3" }</Link<Route>>
            <span>{ " ⏱️ " }</span>
            <span>{ String::from(Date::new_0().to_string()) }</span>
        </nav>
    }
}

#[function_component(Home)]
fn home() -> Html {
    console::log!("this is happening");
    let videos = use_state(Vec::new);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });
    html! {
        <>
            <Nav />
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Post { id } => html! { <Post id={id} /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
