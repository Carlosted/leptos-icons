#[cfg(feature = "BiRegularDotsVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDotsVertical")]
/// *This icon requires the feature* `BiRegularDotsVertical` *to be enabled*.
#[component]
pub fn DotsVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 10h4v4h-4zm0-6h4v4h-4zm0 12h4v4h-4z" /></svg>
   }
}