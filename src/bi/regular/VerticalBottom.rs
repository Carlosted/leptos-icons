#[cfg(feature = "BiRegularVerticalBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularVerticalBottom")]
/// *This icon requires the feature* `BiRegularVerticalBottom` *to be enabled*.
#[component]
pub fn VerticalBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 19h3v2h-3zM13 19h3v2h-3zM8 19h3v2H8zM3 19h3v2H3zM13 5h-2v8H8l4 4 4-4h-3V5z" /></svg>
   }
}