use yew::prelude::*;
use gloo_console::log;

// use yew::services::reader::{FileReader, FileReaderLoadEndEvent};
use crate::components::common::thumbnail::Thumbnail;
use crate::components::common::button::Button;
use crate::components::common::card::Card;

#[function_component]
pub fn UploadPhoto ()-> Html {
    let photo = use_state(|| "images/baby_e.jpg".to_owned());
    let photo_ref = &*photo.clone();

    // let handle_upload = Callback::from(move |e: ChangeData| {
    //     handle_change(e);
    // });

   fn handle_upload(e: Event) {
   

        // let target = e.target().unwrap();
        // let input = target.dyn_ref::<HtmlInputElement>().unwrap();
        // let files = input.files().unwrap();
        // let file = files.get(0).unwrap();
        // let reader = FileReader::new();
        // let reader_c = reader.clone();
        // let onloadend = Closure::wrap(Box::new(move |e: FileReaderLoadEndEvent| {
        //     let result = e.target().result().unwrap();
        //     log!("result: {:?}", result);
        //     // let photo = use_state(|| "/public/baby_e.png".to_owned());
        //     // let photo_ref = &*photo.clone();
        //     // photo_ref.set(result.as_string().unwrap());
        // }) as Box<dyn FnMut(FileReaderLoadEndEvent)>);
        // reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        // reader_c.read_as_text(&file).unwrap();
        // onloadend.forget();
      log!(e)
    }

    html!{
            <div>
                <Thumbnail size={200} src={AttrValue::from(photo_ref.to_string())} alt={"Profile of the user"} class_name={Classes::from("mb-2 mx-auto")}/>
                <a class={classes!(String::from("fs-5 w-100 text-center d-block"))} href="https://www.freepik.com/free-vector/cute-elephant-sitting-waving-hand-cartoon-vector-icon-illustration_11047569.htm#query=baby%20animals&position=39&from_view=search&track=ais">{"Image by catalyststuff"}</a>
                <div>
                    <input type="file" id="upload-photo" name="upload-photo" accept="image/png, image/jpeg" hidden={true} onchange={handle_upload} />
                    <Button class_name={Classes::from("primary d-block mt-2 mx-auto mb-5")}>{"Upload photo"}</Button>
                </div>
            </div>
    }
}