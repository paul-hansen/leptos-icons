#[cfg(feature = "BiRegularRepost")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRepost")]
/// *This icon requires the feature* `BiRegularRepost` *to be enabled*.
#[component]
pub fn Repost(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 7a1 1 0 0 0-1-1h-8v2h7v5h-3l3.969 5L22 13h-3V7zM5 17a1 1 0 0 0 1 1h8v-2H7v-5h3L6 6l-4 5h3v6z" /></svg>
   }
}