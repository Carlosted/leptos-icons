#[cfg(feature = "OcLgRelFilePath")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgRelFilePath")]
/// *This icon requires the feature* `OcLgRelFilePath` *to be enabled*.
#[component]
pub fn RelFilePath(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19.564 4.42a.75.75 0 0 0-1.378-.59l-6.75 15.75a.75.75 0 0 0 1.378.59l6.75-15.75ZM7 18.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z" /></svg>
   }
}