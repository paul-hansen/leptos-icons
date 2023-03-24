#[cfg(feature = "TiPrinter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiPrinter")]
/// *This icon requires the feature* `TiPrinter` *to be enabled*.
#[component]
pub fn Printer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M17 5v-2c0-.552-.448-1-1-1h-9c-.552 0-1 .448-1 1v2c-1.654 0-3 1.346-3 3v10c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-10c0-1.654-1.346-3-3-3zm-9-1h7v5h-7v-5zm-2 3v3c0 .552.448 1 1 1h9c.552 0 1-.448 1-1v-3c.551 0 1 .449 1 1v2.5c0 .827-.673 1.5-1.5 1.5h-10c-.827 0-1.5-.673-1.5-1.5v-2.5c0-.551.449-1 1-1zm11 12h-11c-.551 0-1-.449-1-1v-5.513c.419.318.935.513 1.5.513h10c.565 0 1.081-.195 1.5-.513v5.513c0 .551-.449 1-1 1zM13.5 7h-4c-.276 0-.5.224-.5.5s.224.5.5.5h4c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM15 16h-7c-.276 0-.5.224-.5.5s.224.5.5.5h7c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM13.5 5h-4c-.276 0-.5.224-.5.5s.224.5.5.5h4c.276 0 .5-.224.5-.5s-.224-.5-.5-.5z" /></g></svg>
   }
}