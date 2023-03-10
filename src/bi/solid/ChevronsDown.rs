#[cfg(feature = "BiSolidChevronsDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronsDown")]
/// *This icon requires the feature* `BiSolidChevronsDown` *to be enabled*.
#[component]
pub fn ChevronsDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.939 10.939 12 15.879l-4.939-4.94-2.122 2.122L12 20.121l7.061-7.06z" /><path d="M16.939 3.939 12 8.879l-4.939-4.94-2.122 2.122L12 13.121l7.061-7.06z" /></svg>
   }
}