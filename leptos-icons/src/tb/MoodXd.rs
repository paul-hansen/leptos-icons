#[cfg(feature = "TbMoodXd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodXd")]
/// *This icon requires the feature* `TbMoodXd` *to be enabled*.
#[component]
pub fn MoodXd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-xd" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18z" /><path d="M9 14h6a3 3 0 1 1 -6 0z" /><path d="M9 8l6 3" /><path d="M9 11l6 -3" /></svg>
   }
}