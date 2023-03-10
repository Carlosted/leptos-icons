#[cfg(feature = "BiRegularTrendingDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularTrendingDown")]
/// *This icon requires the feature* `BiRegularTrendingDown` *to be enabled*.
#[component]
pub fn TrendingDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m14 9.586-4 4-6.293-6.293-1.414 1.414L10 16.414l4-4 4.293 4.293L16 19h6v-6l-2.293 2.293z" /></svg>
   }
}