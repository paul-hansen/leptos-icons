#[cfg(feature = "TbVirus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbVirus")]
/// *This icon requires the feature* `TbVirus` *to be enabled*.
#[component]
pub fn Virus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-virus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><path d="M12 7v-4" /><path d="M11 3h2" /><path d="M15.536 8.464l2.828 -2.828" /><path d="M17.657 4.929l1.414 1.414" /><path d="M17 12h4" /><path d="M21 11v2" /><path d="M15.535 15.536l2.829 2.828" /><path d="M19.071 17.657l-1.414 1.414" /><path d="M12 17v4" /><path d="M13 21h-2" /><path d="M8.465 15.536l-2.829 2.828" /><path d="M6.343 19.071l-1.413 -1.414" /><path d="M7 12h-4" /><path d="M3 13v-2" /><path d="M8.464 8.464l-2.828 -2.828" /><path d="M4.929 6.343l1.414 -1.413" /></svg>
   }
}