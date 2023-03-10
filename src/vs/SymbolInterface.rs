#[cfg(feature = "VsSymbolInterface")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolInterface")]
/// *This icon requires the feature* `VsSymbolInterface` *to be enabled*.
#[component]
pub fn SymbolInterface(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M11.496 4a3.49 3.49 0 0 0-3.46 3h-3.1a2 2 0 1 0 0 1h3.1a3.5 3.5 0 1 0 3.46-4zm0 6a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" /></svg>
   }
}