#[cfg(feature = "BiSolidConversation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidConversation")]
/// *This icon requires the feature* `BiSolidConversation` *to be enabled*.
#[component]
pub fn Conversation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 14h3.5c.827 0 1.5-.673 1.5-1.5v-9c0-.827-.673-1.5-1.5-1.5h-13C2.673 2 2 2.673 2 3.5V18l5.333-4H13zm-9-.1.154-.016L4 14v-.1z" /><path d="M20.5 8H20v6.001c0 1.1-.893 1.993-1.99 1.999H8v.5c0 .827.673 1.5 1.5 1.5h7.167L22 22V9.5c0-.827-.673-1.5-1.5-1.5z" /></svg>
   }
}