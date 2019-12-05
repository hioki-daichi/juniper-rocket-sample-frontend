mod common;
mod video;

use common::helper::build_request;
use common::response::ResponseData;
use failure::Error;
use serde_json::json;
use video::model::{NewVideo, Video};
use video::response::VideosResponse;
use yew::format::Json;
use yew::prelude::*;
use yew::services::reader::{FileData, ReaderService, ReaderTask};
use yew::services::{fetch, ConsoleService, FetchService};
use yew::ChangeData;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    new_video: Option<NewVideo>,
    videos: Vec<Video>,
    console: ConsoleService,
    link: ComponentLink<Model>,
    fetch_service: FetchService,
    fetch_task: Option<fetch::FetchTask>,
    reader_service: ReaderService,
    reader_tasks: Vec<ReaderTask>,
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
            new_video: None,
            videos: vec![],
            console: ConsoleService::new(),
            link,
            fetch_service: FetchService::new(),
            fetch_task: None,
            reader_service: ReaderService::new(),
            reader_tasks: vec![],
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

            Msg::ChooseFile(ChangeData::Files(files)) => {
                for file in files {
                    self.reader_tasks.push(
                        self.reader_service
                            .read_file(file, self.link.send_back(move |v| Msg::LoadedFile(v))),
                    );
                }
            }

            Msg::ChooseFile(ChangeData::Value(_)) => {
                self.console.log("ChangeData::Value");
            }

            Msg::ChooseFile(ChangeData::Select(_)) => {
                self.console.log("ChangeData::Select");
            }

            Msg::LoadedFile(file_data) => {
                self.new_video = Some(NewVideo::build(file_data));
            }
        }

        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="container mx-auto">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick=|_| Msg::GetVideos>{ "Get Videos" }</button>
                <input type="file" onchange = |change_data| Msg::ChooseFile(change_data) />
                {{ self.view_new_video(self.new_video.clone()) }}
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

    fn view_new_video(&self, new_video: Option<NewVideo>) -> Html<Self> {
        let a = new_video.unwrap_or(NewVideo {
            src: "".to_string(),
        });

        html! {
          <video class="w-full" autoplay=true muted=true loop=true playsinline=true>
              <source src={a.src.as_str()} />
          </video>
        }
    }
}
