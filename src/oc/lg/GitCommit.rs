#[cfg(feature = "OcLgGitCommit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgGitCommit")]
/// *This icon requires the feature* `OcLgGitCommit` *to be enabled*.
#[component]
pub fn GitCommit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.944 11h4.306a.75.75 0 0 1 0 1.5h-4.306a5.001 5.001 0 0 1-9.888 0H2.75a.75.75 0 0 1 0-1.5h4.306a5.001 5.001 0 0 1 9.888 0Zm-1.444.75a3.5 3.5 0 1 0-7 0 3.5 3.5 0 0 0 7 0Z" /></svg>
   }
}