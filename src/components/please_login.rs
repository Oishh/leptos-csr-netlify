use leptos::*;
#[allow(non_snake_case)]
#[component]
pub fn PleaseLogin() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center p-4 min-h-screen">
            // <div class="dropdown dropdown-right dropdown-end">
            <a href="/login" for="please_login" class="rounded btn btn-md btn-success btn-outline">
                Please Login
            </a>
        // <input hidden type="checkbox" id="please_login" class="modal-toggle" />
        // <div class="modal" role="dialog">
        // <div class="modal-box">
        // <crate::components::login::page::Login/>
        // </div>
        // <label class="modal-backdrop" for="please_login">Close</label>
        // </div>
        // </div>
        </div>
    }
}
