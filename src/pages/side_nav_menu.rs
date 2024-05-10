use super::icons::Icon;
use leptos::*;

const SELECTED_CLASS: &str = "font-normal justify-start capitalize border border-success text-white rounded w-full flex flex-row gap-2 items-center px-2 py-1 hover:bg-success hover:bg-opacity-30";
const UNSELECTED_CLASS: &str = "font-normal justify-start capitalize w-full rounded flex flex-row gap-2 items-center px-2 py-1 hover:bg-success hover:bg-opacity-15";
const SELECTED_CLASS_MOBILE: &str = "flex justify-center border border-success text-white rounded px-2 py-1 hover:bg-success hover:bg-opacity-30";
const UNSELECTED_CLASS_MOBILE: &str = "flex justify-center rounded px-2 py-1 hover:bg-base-100";

#[allow(non_snake_case)]
#[component]
pub fn SideNavigationMainMenu(
    sub_menu_open: bool,
    title: String,
    icon_title: String,
    children: ChildrenFn,
) -> impl IntoView {
    let is_selected_collapse = move |is_selected_collapse: bool| {
        if is_selected_collapse {
            SELECTED_CLASS_MOBILE
        } else {
            UNSELECTED_CLASS_MOBILE
        }
    };

    {
        move || {
            view! {
                <details
                    class="collapse collapse-arrow hidden px924:block rounded"
                    open=move || sub_menu_open
                >
                    <summary class="collapse-title text-sm px-2 py-1 hover:bg-success hover:bg-opacity-15">
                        <div class="flex flex-row gap-2 items-center">
                            <Icon title=icon_title.clone() size="w-5 h-5".to_string()/>
                            // size = "22px".to_string()
                            <span>{format!("{}", title.clone())}</span>
                        </div>
                    </summary>
                    <div class="collapse-content text-xs">{children()}</div>
                </details>

                // ------------------ MOBILE ------------------

                <div class="flex flex-col gap-2 px924:hidden rounded">
                    <div class="dropdown dropdown-right">
                        <div
                            tabindex="0"
                            role="button"
                            class=move || is_selected_collapse(sub_menu_open)
                        >
                            <Icon title=icon_title.clone() size="w-6 h-6".to_string()/>
                        // size = "26px".to_string()
                        </div>
                        <div class="dropdown-content z-[1] menu px-2 py-1 shadow bg-base-100 rounded-box w-52">
                            {children()}
                        </div>
                    </div>
                </div>
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn SideNavigationSubMenu(
    location: String,
    anchor_url: String,
    title: String,
    #[prop(optional)] sub_anchor: bool,
    #[prop(optional)] icon_title: String,
) -> impl IntoView {
    let is_selected = move |is_selected: bool, mobile: bool| {
        if mobile {
            if is_selected {
                SELECTED_CLASS_MOBILE
            } else {
                UNSELECTED_CLASS_MOBILE
            }
        } else {
            if is_selected {
                SELECTED_CLASS
            } else {
                UNSELECTED_CLASS
            }
        }
    };

    let anchor_url_fn = move || format!("{}", anchor_url.clone());
    let location_fn = move || format!("{}", location.clone());

    {
        move || {
            match sub_anchor {
                true => view! {
                    <a
                        class=is_selected(
                            location_fn() == { format!("{}", anchor_url_fn()) },
                            false,
                        )

                        href=format!("{}", anchor_url_fn())
                    >
                        <span class="text-xs text-opacity-70 font-light text-center ml-6">
                            {format!("{}", title.clone())}
                        </span>
                    </a>
                }
                .into_view(),
                false => view! {
                    <div class="text-sm items-center hidden px924:block rounded">
                        <a
                            class=is_selected(
                                location_fn() == { format!("{}", anchor_url_fn()) },
                                false,
                            )

                            href=format!("{}", anchor_url_fn())
                        >
                            <Icon title=icon_title.clone() size="w-5 h-5".to_string()/>
                            // size = "22px".to_string()
                            <span class="text-center">{format!("{}", title.clone())}</span>
                        </a>
                    </div>

                    // ------------------ MOBILE ------------------

                    <div class="flex flex-col gap-2 px924:hidden">
                        <a
                            class=is_selected(
                                location_fn() == { format!("{}", anchor_url_fn()) },
                                true,
                            )

                            href=format!("{}", anchor_url_fn())
                        >
                            <Icon title=icon_title.clone() size="w-6 h-6".to_string()/>
                        // size = "26px".to_string()
                        </a>
                    </div>
                }
                .into_view(),
            }
        }
    }
}
