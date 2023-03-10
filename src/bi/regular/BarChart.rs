#[cfg(feature = "BiRegularBarChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBarChart")]
/// *This icon requires the feature* `BiRegularBarChart` *to be enabled*.
#[component]
pub fn BarChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9 6h2v14H9zm4 2h2v12h-2zm4-4h2v16h-2zM5 12h2v8H5z" /></svg>
   }
}