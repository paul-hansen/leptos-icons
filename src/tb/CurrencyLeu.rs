#[cfg(feature = "TbCurrencyLeu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCurrencyLeu")]
/// *This icon requires the feature* `TbCurrencyLeu` *to be enabled*.
#[component]
pub fn CurrencyLeu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-currency-leu" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M17 18h-7a3 3 0 0 1 -3 -3v-10" /></svg>
   }
}