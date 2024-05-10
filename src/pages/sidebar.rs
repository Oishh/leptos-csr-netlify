use leptos::*;
use leptos_router::use_location;
use super::icons::Icon;
use super::side_nav_menu::{SideNavigationMainMenu, SideNavigationSubMenu};

const UNSELECTED_CLASS: &str = "font-normal justify-start capitalize w-full rounded flex flex-row gap-2 items-center px-2 py-1 hover:bg-success hover:bg-opacity-15";
const UNSELECTED_CLASS_MOBILE: &str = "flex justify-center rounded px-2 py-1 hover:bg-base-100";

#[allow(non_snake_case)]
#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location().pathname;

    // let logout_action = create_action(move |_input: &()| async move {
    //     let result = logout();

    //     match result {
    //         Ok(res) => {
    //             if res {
    //                 true
    //             } else {
    //                 false
    //             }
    //         }
    //         Err(_e) => false,
    //     }
    // });

    let trades_sub_menu_open = move || {
        location.get() == "/trades/positions"
            || location.get() == "/trades/history"
            || location.get() == "/trades/recents"
            || location.get() == "/trades/expiring"
            || location.get() == "/trades/termsheets"
    };

    {
        move || {
            view! {
                <div class="flex flex-col min-h-full justify-between">
                    <div class="p-1 m-1">
                        <div class="flex items-center justify-center px924:justify-start">
                            <div class="flex gap-2 p-2">
                                <Icon title="JABRA".to_string() size="w-7 h-7".to_string()/>
                                <div class="hidden px924:block">
                                    <div class="text-xl font-librebaskerville opacity-70 tracking-wide">
                                        | Jabra.
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="divider divider-ghost mt-0 mb-0"></div>

                        <SideNavigationMainMenu
                            sub_menu_open=location.get() == "/quotes/builder"
                                || location.get() == "/quotes/active"
                            title="Quotes".to_string()
                            icon_title="QUOTES".to_string()
                        >
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/quotes/builder".to_string()
                                title="Quote Builder".to_string()
                                sub_anchor=true
                            />
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/quotes/active".to_string()
                                title="Active Quotes".to_string()
                                sub_anchor=true
                            />
                        </SideNavigationMainMenu>

                        <SideNavigationMainMenu
                            sub_menu_open=trades_sub_menu_open()
                            title="Trades".to_string()
                            icon_title="TRADES".to_string()
                        >
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/trades/positions".to_string()
                                title="Positions".to_string()
                                sub_anchor=true
                            />
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/trades/history".to_string()
                                title="Trade History".to_string()
                                sub_anchor=true
                            />
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/trades/recents".to_string()
                                title="Recent Trades".to_string()
                                sub_anchor=true
                            />
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/trades/expiring".to_string()
                                title="Expiring Trades".to_string()
                                sub_anchor=true
                            />
                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/trades/termsheets".to_string()
                                title="New Term Sheet".to_string()
                                sub_anchor=true
                            />
                        </SideNavigationMainMenu>

                        <SideNavigationSubMenu
                            location=location.get()
                            anchor_url="/riskslide".to_string()
                            title="Risk Slide".to_string()
                            icon_title="RISKSLIDE".to_string()
                        />

                        <SideNavigationSubMenu
                            location=location.get()
                            anchor_url="/counterparties".to_string()
                            title="Counterparties".to_string()
                            icon_title="COUNTERPARTIES".to_string()
                        />

                        <div class="divider divider-ghost mt-0 mb-0"></div>

                        <Suspense>

                            <SideNavigationSubMenu
                                location=location.get()
                                anchor_url="/login".to_string()
                                title="Login".to_string()
                                icon_title="LOGIN".to_string()
                            />

                            // {move || {
                            //     use_context::<crate::CheckCookie>()
                            //         .unwrap()
                            //         .0
                            //         .and_then(|a| {
                            //             if a.clone() == true {
                            //                 let logout_fn = move || {
                            //                     logout_action.dispatch(());
                            //                     use_context::<crate::Refetcher>().unwrap().0.update(|s| *s = !*s);
                            //                 };
                            //                 view! {
                            //                     // <form on:submit=move |_| {
                            //                     //     logout_action.dispatch(());
                            //                     // }>

                            //                         <div class="text-sm items-center hidden px924:block rounded">
                            //                             <button class=UNSELECTED_CLASS on:click=move |_| logout_fn()>
                            //                                 <Icon
                            //                                     title="LOGOUT".to_string()
                            //                                     size="w-5 h-5".to_string()
                            //                                 />
                            //                                 <span class="text-center">Logout</span>
                            //                             </button>
                            //                         </div>

                            //                         // ------------------ MOBILE ------------------

                            //                         <div class="text-sm flex flex-col gap-2 px924:hidden">
                            //                             <button class=UNSELECTED_CLASS_MOBILE on:click=move |_| logout_fn()>
                            //                                 <Icon
                            //                                     title="LOGOUT".to_string()
                            //                                     size="w-6 h-6".to_string()
                            //                                 />
                            //                             </button>
                            //                         </div>
                            //                     // </form>
                            //                 }
                            //                     .into_view()
                            //             } else {
                            //                 view! {
                            //                     // ------------------ MOBILE ------------------

                            //                     // ------------------ MOBILE ------------------

                            //                     // ------------------ MOBILE ------------------

                            //                     <SideNavigationSubMenu
                            //                         location=location.get()
                            //                         anchor_url="/login".to_string()
                            //                         title="Login".to_string()
                            //                         icon_title="LOGIN".to_string()
                            //                     />
                            //                 }
                            //                     .into_view()
                            //             }
                            //         })
                            // }}

                        </Suspense>
                    </div>
                </div>
            }
        }
    }
}

// pub fn logout() -> Result<bool, ServerFnError> {
//     let (_cookie, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>("admin_portal_csr", UseCookieOptions::default().path("/"));
//     set_cookie(Some("".to_string()));

//     Ok(true)
// }
