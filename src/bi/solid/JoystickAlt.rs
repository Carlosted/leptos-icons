#[cfg(feature = "BiSolidJoystickAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidJoystickAlt")]
/// *This icon requires the feature* `BiSolidJoystickAlt` *to be enabled*.
#[component]
pub fn JoystickAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 6H8a6 6 0 0 0 0 12h8a6 6 0 0 0 0-12zm-5 7H9v2H7v-2H5v-2h2V9h2v2h2v2zm3.5 2a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm3-3a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z" /></svg>
   }
}