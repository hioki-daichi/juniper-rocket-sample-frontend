mod common;
mod video;

use common::helper::build_request;
use common::response::ResponseData;
use failure::Error;
use serde_json::json;
use video::model::Video;
use video::response::VideosResponse;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{fetch, ConsoleService, FetchService};

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    videos: Vec<Video>,
    console: ConsoleService,
    link: ComponentLink<Model>,
    fetch_service: FetchService,
    fetch_task: Option<fetch::FetchTask>,
}

enum Msg {
    GetVideos,
    GetVideosSuccess(Vec<Video>),
    GetVideosFailure,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            videos: vec![],
            console: ConsoleService::new(),
            link,
            fetch_service: FetchService::new(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetVideos => {
                self.console.log("Msg::GetVideos");

                let data = json!({ "query": "{ videos { src } }" });
                let request = build_request(&data);

                let callback = self.link.send_back(
                    move |response: fetch::Response<
                        Json<Result<ResponseData<VideosResponse>, Error>>,
                    >| {
                        let (meta, Json(response_body)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::GetVideosSuccess(response_body.unwrap().data.videos)
                        } else {
                            Msg::GetVideosFailure
                        }
                    },
                );

                self.fetch_task = Some(self.fetch_service.fetch(request, callback));
            }

            Msg::GetVideosSuccess(videos) => {
                self.console.log("Msg::GetVideosSuccess");

                self.videos = videos;
            }

            Msg::GetVideosFailure => {
                self.console.log("Msg::GetVideosFailure");

                self.videos = vec![];
            }
        }

        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="container mx-auto">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick=|_| Msg::GetVideos>{ "Get Videos" }</button>
                { for self.videos.iter().map(|video| self.video_view(video)) }
            </div>
        }
    }
}

impl Model {
    fn video_view(&self, video: &Video) -> Html<Self> {
        html! {
            <video class="w-full" autoplay=true muted=true loop=true playsinline=true>
                <source src={video.src.as_str()} />
            </video>
        }
    }
}
