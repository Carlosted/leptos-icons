#[cfg(feature = "HiLgOutlineArrowUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowUp")]
/// *This icon requires the feature* `HiLgOutlineArrowUp` *to be enabled*.
#[component]
pub fn ArrowUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 10.5L12 3M12 3L19.5 10.5M12 3V21" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}