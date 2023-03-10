#[cfg(feature = "AiOutlinedLine")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedLine")]
/// *This icon requires the feature* `AiOutlinedLine` *to be enabled*.
#[component]
pub fn Line(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M904 476H120c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8z" /></svg>
   }
}