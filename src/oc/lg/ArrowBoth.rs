#[cfg(feature = "OcLgArrowBoth")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgArrowBoth")]
/// *This icon requires the feature* `OcLgArrowBoth` *to be enabled*.
#[component]
pub fn ArrowBoth(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7.78 5.97a.75.75 0 0 0-1.06 0l-5.25 5.25a.75.75 0 0 0 0 1.06l5.25 5.25a.75.75 0 0 0 1.06-1.06L3.81 12.5h16.38l-3.97 3.97a.75.75 0 1 0 1.06 1.06l5.25-5.25a.75.75 0 0 0 0-1.06l-5.25-5.25a.75.75 0 1 0-1.06 1.06L20.19 11H3.81l3.97-3.97a.75.75 0 0 0 0-1.06Z" /></svg>
   }
}