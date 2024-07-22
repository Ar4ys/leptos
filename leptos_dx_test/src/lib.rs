#![allow(unused_variables)]
use leptos::*;

#[component]
pub fn Button(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(into)] on_click: Callback<ev::MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
        <button on:click=on_click>
            {children()}
        </button>
    };
}

fn main() {
    #[derive(Clone)]
    struct GameInfo;
    let game_info = GameInfo;
    let is_game_started = move || true;
    let start_game = move |game_info: GameInfo| {};
    let parent_attrs = [()];

    #[allow(deprecated)]
    {
        view! {
            <div class:bg-green-400=is_game_started>
                <Button>"Restart Overlay"</Button>
                <Show when=move || !is_game_started() clone:game_info>
                    <Button on_click=move || start_game(game_info)>"Start Game"</Button>
                </Show>
            </div>
        };
    }

    view! {
        <div class:bg-green-400=is_game_started>
            <Show when=move || !is_game_started() clone:game_info>
                <Button on_click=move |_| start_game(game_info)>"Start Game"</Button>
            </Show>
        </div>
    };

    view! {
        <Button on_click=move |_| start_game(game_info)>
            <Button on_click=move |_| start_game(game_info)>"Start Game"</Button>
        </Button>
    };
}

/// Disable "unreachable_code" warn when there is an error `view!` macro
///
/// Before: the whole `view!` macro was marked as "unreachable_code"
/// After: "unreachable_code" is not reported at all
#[test]
fn unreachable_code() {
    #[component]
    fn Component(_required: i64) -> impl IntoView {}

    view! {
        <Component />
    };
}

/// Report "missing field" and other `.build()` errors on component's name
///
/// Before: The error was reported over the whole `view` macro
/// After: The error is reported over the `Component` name only
/// Regression: "Go to Definition" triggered on `Component` name in `view!` macro now
/// suggests `build` method definition in addition to `Component` itself. Previously
/// only `Component` was reported

#[test]
fn missing_field() {
    #[component]
    fn Component(_required: i64) -> impl IntoView {}

    view! {
        <Component />
    };
}

// FIXME:
/// Unless I remove `#[component]` macro - I cannot get autocomplete for `Children`,
/// or even `leptos::Children`. When I trigger autocomplete for `leptos::C` I get `Clone`
/// as a suggestion.
/// To test - do not forget to comment global `use leptos::*`, otherwise it may seem as if it works.
#[test]
fn no_autocomplete_for_modules() {
    #[component]
    fn Component1(children: Children) -> impl IntoView {}

    #[component]
    fn Component2(children: leptos::Children) -> impl IntoView {}
}

/// Component with props but without `#[component]` macro is used in `view!` macro
///
/// Before: Error reported over the whole `view!` macro
/// After: Error reported over the `Component` name only
#[test]
fn component_with_props_without_macro() {
    #[allow(unused, non_snake_case)]
    fn Component(_a: i32) -> impl IntoView {}

    view! {
        <Component />
    };
}

/// When incorrect generics are passed or inferred in component the error is reported over the whole
/// `view!` macro.
///
/// Before: error is reported over the whole `view!` macro
/// After: error is reported over component's name and it's generics (if any)
/// Regression: "Go to Definition" triggered on `Component` name in `view!` macro now
/// suggests `leptos::component_props_builder` definition in addition to `Component` itself.
#[test]
fn generics() {
    #[component]
    fn Component<T: Into<String>>(_a: T) -> impl IntoView {}

    view! {
        <Component<i32> _a=0 />
    };

    view! {
        <Component _a=0 />
    };
}

/// Error reporting with `#[prop(into)]`
///
/// Before: The error was reported over the whole `view` macro
/// After: The error is reported over the property
#[test]
fn prop_into() {
    #[component]
    fn Component(#[prop(into)] _a: MaybeSignal<String>) -> impl IntoView {}

    view! {
        <Component _a=move || String::new() />
    };

    let _a = move || String::new();

    view! {
        <Component _a />
    };
}

/// Error for "on:" events should be reported on it's name
///
/// Before: Error was reported over the whole `view!` macro
/// After: Error is reported over `on:a`
#[test]
fn events() {
    #[component]
    fn Component() -> impl IntoView {}

    view! {
        <Component on:a=|| {} />
    };
}

/// FIXME: HTML Element directives don't work when in `Mode:Ssr`.
/// Instead they are being treated like plain attributes.
/// In `Mode::Client` they do, however, work.
/// I should ask in leptos discord about that.
#[test]
fn html_elements_directives() {
    fn highlight(el: HtmlElement<html::AnyElement>) {}

    view! {
        <a href="#" use:highlight>"Copy data to clipboard"</a>
    };
}

/// Missing or incorrect type of the param when using directive
///
/// Before: Error is reported over the whole `view!` macro
/// After: Error is reported over directive
#[test]
fn directive() {
    #[component]
    fn Component() -> impl IntoView {}
    fn highlight(el: HtmlElement<html::AnyElement>, a: i32) {}

    let data = "Hello World!";

    view! {
        // BUG: `use` is spanned to `highlight` fn, instead of `highlight` in `use:highlight`.
        // How to fix it - idk, but at least "Go to Definition" on `use` sends you to `highlight` fn.
        <Component use:highlight />
    };

    view! {
        <Component use:highlight="asd" />
    };

    view! {
        <Component use:highlight=data />
    };
}

/// If block inside fragment returns something, that is not "renderable"
///
/// Before: Error is reported over the whole `view!` macro
/// After: Error is only reported over the block itself
#[test]
fn block_in_fragment() {
    struct A;

    view! {
        <>{A}</>
    };
}

/// Error reporting with `#[prop(into)]` in slots
///
/// Before: The error was reported over the whole `view` macro
/// After: The error is reported over the property
#[test]
fn slots_prop_into() {
    #[slot]
    struct ElseIf {
        #[prop(into)]
        cond: MaybeSignal<bool>,
    }

    #[component]
    fn SlotIf(else_if: Vec<ElseIf>) -> impl IntoView {}

    let is_div5 = move || false;

    view! {
        <SlotIf>
            <ElseIf slot cond=is_div5/>
        </SlotIf>
    };
}

/// Wrong slot name
///
/// Before: Error was reported over the whole `view!` macro
/// After: Error is reported over the child slot itself
#[test]
fn wrong_slot_name() {
    #[slot]
    struct Then {}

    #[slot]
    struct ElseIf {}

    #[slot]
    struct SlotIf {
        then: Then,
    }

    #[component]
    fn If(then: Then, slot_if: SlotIf) -> impl IntoView {}

    view! {
        <If>
            <ElseIf slot:then />
            <SlotIf slot>
                <ElseIf slot:then />
            </SlotIf>
        </If>
    };
}

/// Using slot that is not defined in a component
///
/// Before: Error was reported over the component/parent slot name
/// After: Error is reported over the child slot itself
#[test]
fn undefined_slot() {
    #[slot]
    struct Then {}

    #[slot]
    struct ElseIf {
        #[prop(optional)]
        then: Option<Then>,
    }

    #[slot]
    struct SlotIf {
        then: Then,
    }

    #[component]
    fn If(
        then: Then,
        #[prop(optional)] slot_if: Option<SlotIf>,
    ) -> impl IntoView {
    }

    view! {
        <If>
            <ElseIf slot />
            <SlotIf slot>
                <ElseIf slot />
            </SlotIf>
        </If>
    };
}

/// Expected one slot, got more
///
/// Before: Error was reported over the component/parent slot name
/// After: Error is reported over the child slot itself
#[test]
fn slot_expected_one_got_more() {
    #[slot]
    struct Then {}

    #[slot]
    struct SlotIf {
        then: Then,
    }

    #[component]
    fn If(then: Then, slot_if: SlotIf) -> impl IntoView {}

    view! {
        <If>
            <Then slot />
            <Then slot />
            <SlotIf slot>
                <Then slot />
                <Then slot />
            </SlotIf>
        </If>
    };
}

/// Disable "unreachable_code" warn when there is an error in slot in `view!` macro
///
/// Before: the whole `view!` macro was marked as "unreachable_code"
/// After: "unreachable_code" is not reported at all
#[test]
fn slot_unreachable_code() {
    #[slot]
    struct Then {
        a: i32,
    }

    #[component]
    fn SlotIf(then: Then) -> impl IntoView {}

    view! {
        <SlotIf>
            <Then slot />
        </SlotIf>
    };
}

/// Report "missing field" and other `.build()` errors on slot's name
///
/// Before: The error was reported over the whole `view` macro
/// After: The error is reported over the slot's name (`Then`) only
/// Regression: "Go to Definition" triggered on `Then` in `view!` macro now
/// suggests `build` method definition in addition to `Then` itself. Previously
/// only `Then` was reported
#[test]
fn slot_missing_field() {
    #[slot]
    struct Then {
        a: i32,
    }

    #[component]
    fn SlotIf(then: Then) -> impl IntoView {}

    view! {
        <SlotIf>
            // TODO: `<` should not be highlighted green.
            <Then slot />
        </SlotIf>
    };
}

/// TODO: Remove this dumb requirement!
#[test]
fn close_tag_generic_mismatch() {
    #[allow(unused)]
    #[component]
    fn Component<T>(children: T) -> impl IntoView {}

    view! {
        <Component<Children>>
            <p />
        // Who in their mind will force you to duplicate generics?!
        </Component>
    };
}

/// Using `clone:` notation on type that does not implement clone
///
/// Before: The error was reported over the whole `view!` macro
/// After: The error is reported over `clone:a`
#[test]
fn children_clone() {
    #[slot]
    struct Then {
        children: Children,
    }

    #[allow(unused)]
    #[component]
    fn Component(children: Children, then: Then) -> impl IntoView {}

    struct A;
    let a = A;

    view! {
        <Component clone:a>
            <p />
            <Then slot clone:a>
                <p/>
            </Then>
        </Component>
    };
}

/// TODO: Maybe we should report only over the component name?
/// In that case, "Go to Definition" will be even worse, as it will
/// see a "let props" and show it as a possible definition
///
/// Before: Error was reported over the whole `view!` macro
/// After: Error is reported over the whole component
#[test]
fn children_let_expected_one_got_none() {
    #[slot]
    struct Then<T, C, RC>
    where
        C: Fn(T) -> RC,
        RC: IntoView,
    {
        data: T,
        children: C,
    }

    #[component]
    fn Slot<T, C: Fn(T) -> RC, RC: IntoView>(
        then: Then<T, C, RC>,
    ) -> impl IntoView {
    }

    #[component]
    fn Component<T, C: Fn(T) -> RC, RC: IntoView>(
        data: T,
        children: C,
    ) -> impl IntoView {
    }

    view! {
        <Component
            data="".to_owned()
        >
            <p/>
        </Component>
    };

    view! {
        <Slot>
            <Then
                slot
                data="".to_owned()
            >
                <p/>
            </Then>
        </Slot>
    };
}

#[test]
#[allow(unused)]
fn children() {
    fn children_projection_error() {
        #[component]
        fn Outer(children: ChildrenFn) -> impl IntoView {}

        #[component]
        fn Inner(children: ChildrenFn) -> impl IntoView {}

        #[component]
        fn Inmost(name: String) -> impl IntoView {}

        let name = "Alice".to_string();

        view! {
            <Outer>
                <Inner>
                    <Inmost name=name.clone()/>
                </Inner>
            </Outer>
        };
    }

    fn children_projection_error_slot_version() {
        #[component]
        fn Root(outer: Outer) -> impl IntoView {}

        #[slot]
        struct Outer {
            children: ChildrenFn,
        }

        #[component]
        fn Inner(children: ChildrenFn) -> impl IntoView {}

        #[component]
        fn Inmost(name: String) -> impl IntoView {}

        let name = "Alice".to_string();

        view! {
            <Root>
                <Outer slot>
                    <Inner>
                        <Inmost name=name.clone() />
                    </Inner>
                </Outer>
            </Root>
        };
    }

    fn expected_children_got_let_bind() {
        #[component]
        fn Component(children: Children) -> impl IntoView {}

        view! {
            <Component let:a>
                <p />
            </Component>
        };
    }

    fn expected_children_got_let_bind_slot_version() {
        #[slot]
        struct Then {
            children: Children,
        }

        #[allow(unused)]
        #[component]
        fn Component(then: Then) -> impl IntoView {}

        view! {
            <Component>
                <Then slot let:a>
                    <p />
                </Then>
            </Component>
        };
    }

    fn expected_one_let_bind_got_more() {
        #[component]
        fn Component<C, IV>(children: C) -> impl IntoView
        where
            C: Fn(String) -> IV,
            IV: IntoView,
        {
        }

        view! {
            <Component
                let:item
                let:a
            >
                <p>{item}</p>
            </Component>
        };
    }

    fn expected_one_let_bind_got_more_slot_version() {
        #[slot]
        struct Then<C, IV>
        where
            C: Fn(String) -> IV,
            IV: IntoView,
        {
            children: C,
        }

        #[component]
        fn Slot<C, IV>(then: Then<C, IV>) -> impl IntoView
        where
            C: Fn(String) -> IV,
            IV: IntoView,
        {
        }

        view! {
            <Slot>
                <Then
                    slot
                    let:item
                    let:a
                >
                    <p>{item}</p>
                </Then>
            </Slot>
        };
    }
}

// TODO: For some reason in Mode::Client this compiles to nothing and in Mode:Ssr it throws error:
// "Fragment cannot be used inside element"
#[test]
fn fragment_in_element_bug() {
    view! {
        <div>
            <>
                ""
            </>
        </div>
    };
}

/// Incorrect type of the value passed into attribute
///
/// Before: Error was reported over the whole `view!` macro
/// After: Error is reported over the value itself
#[test]
fn element_incorrect_attribute_type() {
    struct A;

    view! {
        <div id=A />
    };

    view! {
        <div on:click=move |_| A />
    };

    view! {
        <div prop:id=A />
    };

    view! {
        <div class:id=A />
    };

    view! {
        <div class=("id", A) />
    };

    view! { class=A,
        <div />
    };

    view! {
        <div style:id=A />
    };

    view! {
        <div style=("id", A) />
    };
}

/// Incorrect type of the spread value
///
/// Before: Error was reported over the whole `view!` macro
/// After: Error is reported over the value itself
#[test]
fn element_incorrect_spread_type() {
    struct A;

    view! {
        <div {..A} />
    };
}

/// Incorrect type of the child
///
/// Before: Error was reported over the whole `view!` macro
/// After: Error is reported over the child itself
#[test]
fn element_incorrect_child_type() {
    struct A;

    view! {
        <div>{A}</div>
    };
}

///
/// Before:
/// After:
#[test]
fn exp() {
    #[component]
    fn Component(a: i32) -> impl IntoView {}

    #[derive(Default)]
    struct A;

    view! {
        <my-element />
    };

    let a: NodeRef<html::Div>;

    view! {
        <div>
            <p/>
            {A}
        </div>
    };

    view! {
        <Component  />
    };
}

// TODO: Finish `root_element_to_tokens_ssr`
