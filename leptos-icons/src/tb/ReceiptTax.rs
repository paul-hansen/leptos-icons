#[cfg(feature = "TbReceiptTax")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbReceiptTax")]
/// *This icon requires the feature* `TbReceiptTax` *to be enabled*.
#[component]
pub fn ReceiptTax(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-receipt-tax" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 14l6 -6" /><circle cx="9.5" cy="8.5" r=".5" fill="currentColor" /><circle cx="14.5" cy="13.5" r=".5" fill="currentColor" /><path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /></svg>
   }
}