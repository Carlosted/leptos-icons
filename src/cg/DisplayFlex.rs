#[cfg(feature = "CgDisplayFlex")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDisplayFlex")]
/// *This icon requires the feature* `CgDisplayFlex` *to be enabled*.
#[component]
pub fn DisplayFlex(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6 17V7H8V17H6Z" fill="currentColor" /><path d="M16 7V17H18V7H16Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M2 3H22V21H2V3ZM4 5V19H20V5H4Z" fill="currentColor" /></svg>
   }
}