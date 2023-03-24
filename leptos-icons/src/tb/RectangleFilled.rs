#[cfg(feature = "TbRectangleFilled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRectangleFilled")]
/// *This icon requires the feature* `TbRectangleFilled` *to be enabled*.
#[component]
pub fn RectangleFilled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-rectangle-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 4h-14a3 3 0 0 0 -3 3v10a3 3 0 0 0 3 3h14a3 3 0 0 0 3 -3v-10a3 3 0 0 0 -3 -3z" stroke-width="0" fill="currentColor" /></svg>
   }
}