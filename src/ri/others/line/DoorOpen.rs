#[cfg(feature = "RiOthersLineDoorOpen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiOthersLineDoorOpen")]
/// *This icon requires the feature* `RiOthersLineDoorOpen` *to be enabled*.
#[component]
pub fn DoorOpen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M2 21v-2h2V4.835c0-.484.346-.898.821-.984l9.472-1.722c.326-.06.638.157.697.483.007.035.01.07.01.107v1.28L19 4c.552 0 1 .448 1 1v14h2v2h-4V6h-3v15H2zM13 4.396L6 5.67V19h7V4.396zM12 11v2h-2v-2h2z" /></g></svg>
   }
}