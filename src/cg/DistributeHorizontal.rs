#[cfg(feature = "CgDistributeHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDistributeHorizontal")]
/// *This icon requires the feature* `CgDistributeHorizontal` *to be enabled*.
#[component]
pub fn DistributeHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11 9H13V15H11V9Z" stroke="currentColor" stroke-opacity="0.5" stroke-width="2" /><path d="M5 5V19H7V5H5Z" fill="currentColor" /><path d="M17 5V19H19V5H17Z" fill="currentColor" /></svg>
   }
}