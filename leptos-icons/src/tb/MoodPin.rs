#[cfg(feature = "TbMoodPin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodPin")]
/// *This icon requires the feature* `TbMoodPin` *to be enabled*.
#[component]
pub fn MoodPin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-pin" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 18v.01m2 -6.01a9 9 0 1 0 -8.34 8.976" /><path d="M9.5 15a3.59 3.59 0 0 0 2.796 .988" /><path d="M21.121 20.121a3 3 0 1 0 -4.242 0l2.121 2.122l2.121 -2.122z" /><path d="M9 10h.01" /><path d="M15 10h.01" /></svg>
   }
}