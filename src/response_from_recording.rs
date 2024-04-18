use std::fs::create_dir_all;
use std::io::Read;
use std::path::PathBuf;
use reqwest::Body;
use crate::{Request, Respond, ResponseTemplate};

pub struct ResponseFromRecording {
    lazy_record_from_server: String,
    default_recording_folder: PathBuf
}

impl ResponseFromRecording {
    pub fn lazy_record(uri: &str) -> Self {
        Self {
            lazy_record_from_server: uri.to_string(),
            default_recording_folder: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("recorded_responses")
        }
    }
}
impl Respond for ResponseFromRecording {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        create_dir_all(&self.default_recording_folder).ok(); // fixme: error handling

        dbg!(&request.url);
        match request.method.as_str() {
            "POST" => {
                let client =reqwest::blocking::Client::new();
                let proxy_req = client.post(format!("{}{}", &self.lazy_record_from_server, request.url.path()))
                    .body((&request.body).to_vec())
                    .build()
                    .unwrap(); // fixme: error handling
                let response = client.execute(proxy_req)
                    .unwrap(); // fixme: error handling

                ResponseTemplate::new(response.status().as_u16())
                    .set_body_bytes(response.bytes().unwrap())
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
