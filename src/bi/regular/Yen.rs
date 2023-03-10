#[cfg(feature = "BiRegularYen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularYen")]
/// *This icon requires the feature* `BiRegularYen` *to be enabled*.
#[component]
pub fn Yen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17.2 3.4 12 10.333 6.8 3.4 5.2 4.6 10 11H7v2h4v2H7v2h4v4h2v-4h4v-2h-4v-2h4v-2h-3l4.8-6.4z" /></svg>
   }
}