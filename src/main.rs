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
use yew::services::{
    fetch::{FetchService, FetchTask, Response},
    reader::{FileData, ReaderService, ReaderTask},
    ConsoleService,
};
use yew::ChangeData;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    link: ComponentLink<Model>,
    #[allow(dead_code)]
    console: ConsoleService,
    reader_service: ReaderService,
    reader_tasks: Vec<ReaderTask>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    videos: Vec<Video>,
}

enum Msg {
    GetVideos,
    GetVideosSuccess(Vec<Video>),
    GetVideosFailure,
    ChooseFile(ChangeData),
    LoadedFile(FileData),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            console: ConsoleService::new(),
            reader_service: ReaderService::new(),
            reader_tasks: vec![],
            fetch_service: FetchService::new(),
            fetch_task: None,
            videos: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetVideos => {
                let data = json!({ "query": "{ videos { src } }" });

                let request = build_request(&data);

                let callback = self.link.send_back(
                    move |response: Response<Json<Result<ResponseData<VideosResponse>, Error>>>| {
                        let (meta, Json(response_body)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::GetVideosSuccess(response_body.unwrap().data.videos)
                        } else {
                            Msg::GetVideosFailure
                        }
                    },
                );

                let fetch_task = self.fetch_service.fetch(request, callback);

                self.fetch_task = Some(fetch_task);
            }

            Msg::GetVideosSuccess(videos) => {
                self.videos = videos;
            }

            Msg::GetVideosFailure => {
                self.videos = vec![];
            }

            Msg::ChooseFile(change_data) => {
                if let ChangeData::Files(files) = change_data {
                    for file in files {
                        let callback = self
                            .link
                            .send_back(move |file_data| Msg::LoadedFile(file_data));
                        let reader_task = self.reader_service.read_file(file, callback);
                        self.reader_tasks.push(reader_task);
                    }
                }
            }

            Msg::LoadedFile(file_data) => {
                let video = Video::new(file_data);
                self.videos.push(video);
                // TODO: Send Video Mutation Request
            }
        }

        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="container mx-auto">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick=|_| Msg::GetVideos>{ "Get Videos" }</button>
                <input type="file" onchange = |change_data| Msg::ChooseFile(change_data) />
                { for self.videos.iter().map(|video| self.view_video(video)) }
            </div>
        }
    }
}

impl Model {
    fn view_video(&self, video: &Video) -> Html<Self> {
        html! {
            <video class="w-full" autoplay=true muted=true loop=true playsinline=true>
                <source src={video.src.as_str()} />
            </video>
        }
    }
}
