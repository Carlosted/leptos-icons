#[cfg(feature = "BiSolidPlusSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPlusSquare")]
/// *This icon requires the feature* `BiSolidPlusSquare` *to be enabled*.
#[component]
pub fn PlusSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2zm2-10h4V7h2v4h4v2h-4v4h-2v-4H7v-2z" /></svg>
   }
}