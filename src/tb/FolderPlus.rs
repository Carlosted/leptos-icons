#[cfg(feature = "TbFolderPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFolderPlus")]
/// *This icon requires the feature* `TbFolderPlus` *to be enabled*.
#[component]
pub fn FolderPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" /><path d="M12 10l0 6" /><path d="M9 13l6 0" /></svg>
   }
}