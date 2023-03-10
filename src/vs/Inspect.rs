#[cfg(feature = "VsInspect")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsInspect")]
/// *This icon requires the feature* `VsInspect` *to be enabled*.
#[component]
pub fn Inspect(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1 3l1-1h12l1 1v6h-1V3H2v8h5v1H2l-1-1V3zm14.707 9.707L9 6v9.414l2.707-2.707h4zM10 13V8.414l3.293 3.293h-2L10 13z" /></svg>
   }
}