#[cfg(feature = "BiRegularRightIndent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRightIndent")]
/// *This icon requires the feature* `BiRegularRightIndent` *to be enabled*.
#[component]
pub fn RightIndent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 15h10v2H10zm-6 4h16v2H4zm6-8h10v2H10zm0-4h10v2H10zM4 3h16v2H4zm0 5v8l4-4z" /></svg>
   }
}