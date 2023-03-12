#[cfg(feature = "IoPlayCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlayCircleSharp")]
/// *This icon requires the feature* `IoPlayCircleSharp` *to be enabled*.
#[component]
pub fn PlayCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM200,344V168l144,88Z" /></svg>
   }
}