#[cfg(feature = "TiRadar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiRadar")]
/// *This icon requires the feature* `TiRadar` *to be enabled*.
#[component]
pub fn Radar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M12 20c3.86 0 7-3.141 7-7s-3.14-7-7.003-7c-3.858 0-6.997 3.141-6.997 7s3.14 7 7 7zm-1-11.898v1.898c0 .553.448 1 1 1s1-.447 1-1v-1.898c1.956.398 3.5 1.942 3.899 3.898h-1.899c-.552 0-1 .447-1 1s.448 1 1 1h1.899c-.399 1.956-1.943 3.5-3.899 3.898v-1.898c0-.553-.448-1-1-1s-1 .447-1 1v1.898c-1.956-.398-3.5-1.942-3.899-3.898h1.899c.552 0 1-.447 1-1s-.448-1-1-1h-1.899c.399-1.956 1.942-3.5 3.899-3.898z" /></svg>
   }
}