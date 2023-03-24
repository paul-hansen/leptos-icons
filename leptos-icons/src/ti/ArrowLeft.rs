#[cfg(feature = "TiArrowLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiArrowLeft")]
/// *This icon requires the feature* `TiArrowLeft` *to be enabled*.
#[component]
pub fn ArrowLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M17 11h-7.586l2.293-2.293c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-4.707 4.707 4.707 4.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-2.293-2.293h7.586c.552 0 1-.448 1-1s-.448-1-1-1z" /></svg>
   }
}