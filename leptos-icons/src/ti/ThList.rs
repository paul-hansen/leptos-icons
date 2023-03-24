#[cfg(feature = "TiThList")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiThList")]
/// *This icon requires the feature* `TiThList` *to be enabled*.
#[component]
pub fn ThList(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M19 17h-7c-1.103 0-2 .897-2 2s.897 2 2 2h7c1.103 0 2-.897 2-2s-.897-2-2-2zM19 10h-7c-1.103 0-2 .897-2 2s.897 2 2 2h7c1.103 0 2-.897 2-2s-.897-2-2-2zM19 3h-7c-1.103 0-2 .897-2 2s.897 2 2 2h7c1.103 0 2-.897 2-2s-.897-2-2-2z" /><circle cx="5" cy="19" r="2.5" /><circle cx="5" cy="12" r="2.5" /><circle cx="5" cy="5" r="2.5" /></svg>
   }
}