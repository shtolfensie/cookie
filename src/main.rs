use leptos::{*, html::Input, leptos_dom::logging::console_log};
use web_sys::SubmitEvent;




#[derive(Debug, Clone)]
struct Ingredient {
    id: u32,
    name: String,
    quantity: Option<String>,
    certainty: Option<String>,
}

// type IngredientsWithId= Vec<(u32, Ingredient)>;

// struct IngredientList {
//     list: Vec<(u32, Ingredient)>,
//     last_id: u32
// }

// impl IngredientList {
//     fn push(&mut self, ingredient: Ingredient) {
//         self.last_id += 1;
//         self.list.push((self.last_id, ingredient))
//     }
// }



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
            <Navbar/>

            <div class="mt-20 flex flex-col md:flex-row gap-2 lg:gap-8 px-2 md:px-5 lg:px-12" >
                <div class="w-full md:w-2/5" >
                    <Pantry/>
                </div>

                <div class="flex-grow" >
                    <Pantry/>
                </div>
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
    view! { <progress max=max value=progress></progress> }
}

#[component]
fn Button(
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="text-white bg-gradient-to-r from-purple-500 to-pink-500 hover:bg-gradient-to-l focus:ring-4 focus:outline-none focus:ring-purple-200 dark:focus:ring-purple-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2"
        >
            {children()}
        </button>
    }
}


#[component]
fn Pantry() -> impl IntoView {

    let (ingredients, set_ingredients) = create_signal(vec![ Ingredient { id: 0, name: "Potatoes".to_owned(), quantity: None, certainty: None } ]);
    let (last_ingredient_id, set_last_ingredient_id) = create_signal(0);

    let i = ingredients;


    let on_ingredient_add = move |i: Ingredient| {
        set_last_ingredient_id.update(|n| *n += 1);
        set_ingredients.update(|data| data.push(Ingredient { id: last_ingredient_id(), name: i.name, quantity: None, certainty: None }));
    };

    view! {
        <div class="w-full p-2 bg-white border border-gray-200 rounded-lg shadow md:p-4 dark:bg-gray-800 dark:border-gray-700">
            <h5 class="text-xl font-medium text-gray-900 dark:text-white">"Pantry"</h5>
            <div class="flex flex-col gap-1">
                <div class="flex flex-col gap-1" >
                    <IngredientList ingredients=ingredients />
                </div>

                <IngredientInput on_add=on_ingredient_add />
            </div>

        </div>
    }
}

#[component]
fn IngredientItem(
    ingredient: Ingredient
) -> impl IntoView {
    view! {
        <li class="py-3 sm:py-4" >
            <div class="flex items-center space-x-3 rtl:space-x-reverse" >
                <div class="flex-shrink-0" >
                    <img class="w-8 h-8 rounded-full" src="/images/potato.png" alt="Image of a potato"/>
                </div>
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-semibold text-gray-900 truncate dark:text-white">
                        {ingredient.name}
                    </p>
                    <p class="text-sm text-gray-500 truncate dark:text-gray-400">
                        {ingredient.quantity}
                    </p>
                </div>
                <span class="inline-flex items-center bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-green-900 dark:text-green-300">
                    <span class="w-2 h-2 me-1 bg-green-500 rounded-full"></span>
                        {ingredient.certainty.unwrap_or("Have".to_owned())}
                </span>
            </div>
        </li>
    }
}

#[component]
fn IngredientList(ingredients: ReadSignal<Vec<Ingredient>>) -> impl IntoView {
    view! {

        <ul role="list" class="w-full divide-y divide-gray-200 dark:divide-gray-700" >
            <For
                each=ingredients
                key=|i| i.id.clone()
                let:child
            >
            <IngredientItem ingredient=child />
        </For>
        </ul>
    }
}

#[component]
fn IngredientInput(#[prop(into)] on_add: Callback<Ingredient>) -> impl IntoView {

    let input_el: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let input = input_el().expect("<input> to exist");
        let value = input.value();

        on_add(Ingredient {id:0, name: value, quantity: None, certainty: None });

        input.set_value("");
    };
    
    view! {
        <form on:submit=on_submit>
            <div class="flex flex-row gap-1" >
                <input
                    type="text"
                    name="new-item"
                    id="new-item"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"
                    placeholder="Potatoes"
                    required
                    node_ref=input_el
                />
                <AddButton w=6 h=6 btn_type="submit".to_owned() />
            </div>
        </form>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-white border-gray-200 dark:bg-gray-900">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a href="#" class="flex items-center space-x-3 rtl:space-x-reverse">
                    // <img src="" class="h-8" alt="Cookie logo" />
                    <CookieLogo />
                    <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">
                        "Cookie"
                    </span>
                </a>
                <div class="flex items-center md:order-2 space-x-3 md:space-x-0 rtl:space-x-reverse">
                    <button
                        type="button"
                        class="flex text-sm bg-gray-800 rounded-full md:me-0 focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600"
                        id="user-menu-button"
                        aria-expanded="false"
                        data-dropdown-toggle="user-dropdown"
                        data-dropdown-placement="bottom"
                    >
                        <span class="sr-only">"Open user menu"</span>
                        <img class="w-8 h-8 rounded-full" src="" alt="user photo"/>
                    </button>
                    // Dropdown menu
                    <div
                        class="z-50 hidden my-4 text-base list-none bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 dark:divide-gray-600"
                        id="user-dropdown"
                    >
                        <div class="px-4 py-3">
                            <span class="block text-sm text-gray-900 dark:text-white">
                                "Bonnie Green"
                            </span>
                            <span class="block text-sm  text-gray-500 truncate dark:text-gray-400">
                                "name@flowbite.com"
                            </span>
                        </div>
                        <ul class="py-2" aria-labelledby="user-menu-button">
                            <li>
                                <a
                                    href="#"
                                    class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                >
                                    "Settings"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="#"
                                    class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                >
                                    "Sign out"
                                </a>
                            </li>
                        </ul>
                    </div>
                    <button
                        data-collapse-toggle="navbar-user"
                        type="button"
                        class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                        aria-controls="navbar-user"
                        aria-expanded="false"
                    >
                        <span class="sr-only">"Open main menu"</span>
                        <svg
                            class="w-5 h-5"
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 17 14"
                        >
                            <path
                                stroke="currentColor"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M1 1h15M1 7h15M1 13h15"
                            ></path>
                        </svg>
                    </button>
                </div>
                <div
                    class="items-center justify-between hidden w-full md:flex md:w-auto md:order-1"
                    id="navbar-user"
                >
                    <ul class="flex flex-col font-medium p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                        <li>
                            <a
                                href="#"
                                class="block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500"
                                aria-current="page"
                            >
                                "Lab"
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700"
                            >
                                "Book"
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn AddButton(
    #[prop(default = 5)]
    w: u16,
    #[prop(default = 5)]
    h: u16,
    #[prop(default = "button".to_owned())]
    btn_type: String
) -> impl IntoView {
    view! {
        <button type={btn_type} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm p-2 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
            // <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
            //     <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5h12m0 0L9 1m4 4L9 9"/>
            // </svg>
            // <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 16">
            //     <path stroke="currentColor" d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
            // </svg>

            <svg class={"w-".to_owned()+&w.to_string()+" h-"+&h.to_string()} xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 16">
                <path stroke="currentColor" d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
            </svg>

            <span class="sr-only">"Add ingredient button"</span>
        </button>

    }
}

#[component]
fn CookieLogo() -> impl IntoView {
    view! {
        <svg class="w-7 h-7" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" alt="Cookie Logo">
            <circle cx="100" cy="100" r="80" fill="#d2a679" />

            <circle cx="60" cy="60" r="10" fill="#3e2723" />
            <circle cx="90" cy="40" r="12"   fill="#3e2723" />
            <circle cx="120" cy="70" r="14"  fill="#3e2723" />
            <circle cx="150" cy="50" r="11"  fill="#3e2723" />
            <circle cx="170" cy="80" r="13"  fill="#3e2723" />
            <circle cx="50" cy="110" r="15"  fill="#3e2723" />
            <circle cx="80" cy="140" r="16"  fill="#3e2723" />
            <circle cx="110" cy="120" r="18" fill="#3e2723" />
            <circle cx="140" cy="150" r="14" fill="#3e2723" />
        </svg>
    }
}
