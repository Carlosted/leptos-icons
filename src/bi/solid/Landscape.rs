#[cfg(feature = "BiSolidLandscape")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLandscape")]
/// *This icon requires the feature* `BiSolidLandscape` *to be enabled*.
#[component]
pub fn Landscape(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="6.5" cy="6.5" r="2.5" /><path d="m14 7-5.223 8.487L7 13l-5 7h20z" /></svg>
   }
}