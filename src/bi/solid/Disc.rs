#[cfg(feature = "BiSolidDisc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDisc")]
/// *This icon requires the feature* `BiSolidDisc` *to be enabled*.
#[component]
pub fn Disc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 14c-2.206 0-4-1.794-4-4s1.794-4 4-4 4 1.794 4 4-1.794 4-4 4z" /><circle cx="11.998" cy="11.998" r="2.002" /></svg>
   }
}