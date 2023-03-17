#[cfg(feature = "RiMapFillNavigation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillNavigation")]
/// *This icon requires the feature* `RiMapFillNavigation` *to be enabled*.
#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2.9 2.3l18.805 6.268a.5.5 0 0 1 .028.939L13 13l-4.425 8.85a.5.5 0 0 1-.928-.086L2.26 2.911A.5.5 0 0 1 2.9 2.3z" /></g></svg>
   }
}