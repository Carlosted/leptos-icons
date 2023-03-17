#[cfg(feature = "BiSolidMouse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMouse")]
/// *This icon requires the feature* `BiSolidMouse` *to be enabled*.
#[component]
pub fn Mouse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.975 22H12c3.859 0 7-3.14 7-7V9c0-3.841-3.127-6.974-6.981-7h-.06C8.119 2.022 5 5.157 5 9v6c0 3.86 3.129 7 6.975 7zM11 6h2v6h-2V6z" /></svg>
   }
}