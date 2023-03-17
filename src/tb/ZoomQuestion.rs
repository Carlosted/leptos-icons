#[cfg(feature = "TbZoomQuestion")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbZoomQuestion")]
/// *This icon requires the feature* `TbZoomQuestion` *to be enabled*.
#[component]
pub fn ZoomQuestion(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-zoom-question" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /><path d="M21 21l-6 -6" /><path d="M10 13l0 .01" /><path d="M10 10a1.5 1.5 0 1 0 -1.14 -2.474" /></svg>
   }
}