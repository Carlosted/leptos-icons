#[cfg(feature = "BiRegularLibrary")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLibrary")]
/// *This icon requires the feature* `BiRegularLibrary` *to be enabled*.
#[component]
pub fn Library(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 3h2v18H7zM4 3h2v18H4zm6 0h2v18h-2zm9.062 17.792-6.223-16.89 1.877-.692 6.223 16.89z" /></svg>
   }
}