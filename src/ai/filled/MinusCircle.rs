#[cfg(feature = "AiFilledMinusCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledMinusCircle")]
/// *This icon requires the feature* `AiFilledMinusCircle` *to be enabled*.
#[component]
pub fn MinusCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm192 472c0 4.4-3.6 8-8 8H328c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h368c4.4 0 8 3.6 8 8v48z" /></svg>
   }
}