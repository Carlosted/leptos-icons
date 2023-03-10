#[cfg(feature = "VsTasklist")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTasklist")]
/// *This icon requires the feature* `VsTasklist` *to be enabled*.
#[component]
pub fn Tasklist(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M3.57 6.699l5.693-4.936L8.585 1 3.273 5.596l-1.51-1.832L1 4.442l1.85 2.214.72.043zM15 5H6.824l2.307-2H15v2zM6 7h9v2H6V7zm9 4H6v2h9v-2z" /></svg>
   }
}