#[cfg(feature = "TiClipboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiClipboard")]
/// *This icon requires the feature* `TiClipboard` *to be enabled*.
#[component]
pub fn Clipboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M17 3h-10c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-12c0-1.654-1.346-3-3-3zm-8 2h6v1c0 .551-.449 1-1 1h-4c-.551 0-1-.449-1-1v-1zm9 13c0 .551-.449 1-1 1h-10c-.551 0-1-.449-1-1v-12c0-.551.449-1 1-1h1v1c0 1.1.9 2 2 2h4c1.1 0 2-.9 2-2v-1h1c.551 0 1 .449 1 1v12zM16 17h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 14h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 11h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5z" /></g></svg>
   }
}