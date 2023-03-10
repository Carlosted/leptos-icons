#[cfg(feature = "BiSolidArrowFromLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowFromLeft")]
/// *This icon requires the feature* `BiSolidArrowFromLeft` *to be enabled*.
#[component]
pub fn ArrowFromLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h2v12H4zm10 5H8v2h6v5l6-6-6-6z" /></svg>
   }
}