#[cfg(feature = "VsSplitHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSplitHorizontal")]
/// *This icon requires the feature* `VsSplitHorizontal` *to be enabled*.
#[component]
pub fn SplitHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14 1H3L2 2v11l1 1h11l1-1V2l-1-1zM8 13H3V2h5v11zm6 0H9V2h5v11z" /></svg>
   }
}