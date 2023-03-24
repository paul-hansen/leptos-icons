#[cfg(feature = "TbMedicineSyrup")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMedicineSyrup")]
/// *This icon requires the feature* `TbMedicineSyrup` *to be enabled*.
#[component]
pub fn MedicineSyrup(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-medicine-syrup" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 21h8a1 1 0 0 0 1 -1v-10a3 3 0 0 0 -3 -3h-4a3 3 0 0 0 -3 3v10a1 1 0 0 0 1 1z" /><path d="M10 14h4" /><path d="M12 12v4" /><path d="M10 7v-3a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v3" /></svg>
   }
}