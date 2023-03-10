#[cfg(feature = "ImTable")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTable")]
/// *This icon requires the feature* `ImTable` *to be enabled*.
#[component]
pub fn Table(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 3v11h16v-11h-16zM6 10v-2h4v2h-4zM10 11v2h-4v-2h4zM10 5v2h-4v-2h4zM5 5v2h-4v-2h4zM1 8h4v2h-4v-2zM11 8h4v2h-4v-2zM11 7v-2h4v2h-4zM1 11h4v2h-4v-2zM11 13v-2h4v2h-4z" /></svg>
   }
}