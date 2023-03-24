#[cfg(feature = "TiArrowRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiArrowRight")]
/// *This icon requires the feature* `TiArrowRight` *to be enabled*.
#[component]
pub fn ArrowRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M13.293 7.293c-.391.391-.391 1.023 0 1.414l2.293 2.293h-7.586c-.552 0-1 .448-1 1s.448 1 1 1h7.586l-2.293 2.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l4.707-4.707-4.707-4.707c-.391-.391-1.023-.391-1.414 0z" /></svg>
   }
}