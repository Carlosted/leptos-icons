#[cfg(feature = "HiLgOutlineChevronDoubleDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronDoubleDown")]
/// *This icon requires the feature* `HiLgOutlineChevronDoubleDown` *to be enabled*.
#[component]
pub fn ChevronDoubleDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19.5 5.25L12 12.75L4.5 5.25M19.5 11.25L12 18.75L4.5 11.25" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}