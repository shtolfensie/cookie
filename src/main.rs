use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {

    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);

    let double_count = move || count() * 2;


    view! {
        <>
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class=("red", move || count() % 2 == 1)
        >
            "Click me: "
            {count}
        </button>
        <ProgressBar progress=count max=50 />
        <ProgressBar progress=Signal::derive(double_count) />
        <div class="text-red-900 pt-5">
            "Hello there"
        </div>
        </>
    }

}

/// Shows a progress bar
#[component]
fn ProgressBar(
    /// the maximum value of the bar
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
