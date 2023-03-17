#[cfg(feature = "BiSolidUniversalAccess")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUniversalAccess")]
/// *This icon requires the feature* `BiSolidUniversalAccess` *to be enabled*.
#[component]
pub fn UniversalAccess(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 3.33A1.67 1.67 0 1 1 10.33 7 1.67 1.67 0 0 1 12 5.33zm3.33 12.5-1.66.84-1.39-3.89h-.56l-1.39 3.89-1.66-.84 1.66-4.72v-1.66L7 10.33l.56-1.66 3.33 1.11h2.22l3.33-1.11.56 1.66-3.33 1.12v1.66z" /></svg>
   }
}