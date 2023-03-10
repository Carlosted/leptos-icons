#[cfg(feature = "OcLgHorizontalRule")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgHorizontalRule")]
/// *This icon requires the feature* `OcLgHorizontalRule` *to be enabled*.
#[component]
pub fn HorizontalRule(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M2 12.75a.75.75 0 0 1 .75-.75h18.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z" /></svg>
   }
}