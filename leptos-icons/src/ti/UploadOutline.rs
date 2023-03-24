#[cfg(feature = "TiUploadOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiUploadOutline")]
/// *This icon requires the feature* `TiUploadOutline` *to be enabled*.
#[component]
pub fn UploadOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M20.986 17c0-.105-.004-.211-.038-.316l-2-6c-.093-.276-.302-.483-.56-.594.881-1.175.799-2.847-.269-3.914l-6.119-6.121-6.121 6.121c-1.067 1.067-1.149 2.739-.27 3.914-.256.109-.467.316-.559.594l-2 6c-.034.105-.038.211-.038.316-.012 0-.012 5-.012 5 0 .553.447 1 1 1h16c.553 0 1-.447 1-1 0 0 0-5-.014-5zm-13.693-9.41l4.707-4.707 4.707 4.707c.391.391.391 1.023 0 1.414-.379.377-1.035.377-1.414 0l-2.293-2.293v5.789c0 .552-.448 1-1 1s-1-.448-1-1v-5.789l-2.293 2.293c-.379.377-1.035.377-1.414 0-.391-.391-.391-1.025 0-1.414zm-.572 4.41h2.279v.5c0 1.654 1.346 3 3 3s3-1.346 3-3v-.5h2.279l1.666 5h-13.892l1.668-5zm-1.721 9v-3h14v3h-14z" /></svg>
   }
}