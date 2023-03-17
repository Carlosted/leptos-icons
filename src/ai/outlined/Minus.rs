#[cfg(feature = "AiOutlinedMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedMinus")]
/// *This icon requires the feature* `AiOutlinedMinus` *to be enabled*.
#[component]
pub fn Minus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M872 474H152c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h720c4.4 0 8-3.6 8-8v-60c0-4.4-3.6-8-8-8z" /></svg>
   }
}