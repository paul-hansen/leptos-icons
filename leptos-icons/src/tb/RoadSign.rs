#[cfg(feature = "TbRoadSign")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRoadSign")]
/// *This icon requires the feature* `TbRoadSign` *to be enabled*.
#[component]
pub fn RoadSign(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-road-sign" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13.446 2.6l7.955 7.954a2.045 2.045 0 0 1 0 2.892l-7.955 7.955a2.045 2.045 0 0 1 -2.892 0l-7.955 -7.955a2.045 2.045 0 0 1 0 -2.892l7.955 -7.955a2.045 2.045 0 0 1 2.892 0z" /><path d="M9 14v-2c0 -.59 .414 -1 1 -1h5" /><path d="M13 9l2 2l-2 2" /></svg>
   }
}