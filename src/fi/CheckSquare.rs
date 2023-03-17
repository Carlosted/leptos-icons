#[cfg(feature = "FiCheckSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCheckSquare")]
/// *This icon requires the feature* `FiCheckSquare` *to be enabled*.
#[component]
pub fn CheckSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 11 12 14 22 4" /><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" /></svg>
   }
}