#[cfg(feature = "BiRegularPopsicle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPopsicle")]
/// *This icon requires the feature* `BiRegularPopsicle` *to be enabled*.
#[component]
pub fn Popsicle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M15 2a7 7 0 0 0-4.94 2l-7.78 7.82a1 1 0 0 0 0 1.41l3.54 3.54-3.54 3.53 1.42 1.42 3.53-3.54 3.54 3.54a1 1 0 0 0 1.41 0L20 13.94A7 7 0 0 0 15 2zm3.54 10.54-7.07 7.06-2.82-2.83-1.42-1.42-2.83-2.83 7.07-7.07a5 5 0 0 1 7.08 7.08z" /></svg>
   }
}