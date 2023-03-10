#[cfg(feature = "SiWise")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiWise")]
/// *This icon requires the feature* `SiWise` *to be enabled*.
#[component]
pub fn Wise(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="m3.6426 0 3.7383 6.3594-6.6602 6.3613H12.043l1.1816-2.7734H7.4883l3.5879-3.588-2.084-3.5878h9.7324L9.7441 24h3.373L23.2794 0Z" /></svg>
   }
}