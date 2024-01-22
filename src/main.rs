use leptos::{*, html::Input, leptos_dom::logging::console_log};
use uuid::Uuid;
use web_sys::{SubmitEvent, MouseEvent};




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


#[derive(Debug, Clone)]
struct RecipeItem {
    id: Uuid,
    text: String,
}


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

                <div class="w-full md:w-3/5" >
                    // <RecipeList recipes=recipes/>
                    <RecipeList />
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


    let on_ingredient_add = move |i: Ingredient| {
        set_last_ingredient_id.update(|n| *n += 1);
        set_ingredients.update(|data| data.push(Ingredient { id: last_ingredient_id(), name: i.name, quantity: None, certainty: None }));
    };

    provide_context(set_ingredients);

    view! {
        <div class="w-full p-2 bg-white border border-gray-200 rounded-lg shadow md:p-4 dark:bg-gray-800 dark:border-gray-700 text-white">
            <h5 class="text-xl font-medium text-gray-900 dark:text-white">"Pantry"</h5>
            <div class="flex flex-col gap-1" >
                <div class="flex flex-col gap-1" >
                    <Show
                        when=move || { !ingredients().is_empty() }
                        fallback=|| view! { <p class="my-5 text-gray-300">"There seems to be nothing here..."</p> }
                    >
                        <IngredientList ingredients=ingredients />
                    </Show>
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

    let ingredient_setter = use_context::<WriteSignal<Vec<Ingredient>>>();

    let handle_delete = move |ev: MouseEvent| {
        ev.prevent_default();

        if let Some(setter) = ingredient_setter {
            setter.update(|ings|  ings.retain(|i| i.id != ingredient.id));
        }
    };

    view! {
        <li class="py-3 sm:py-4 relative group">
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

            <div class="transition-opacity absolute top-[calc(50%-13px)] -right-2.5 opacity-0 group-hover:opacity-100 group-focus:opacity-100 has-[:focus]:opacity-100" >
                <DeleteButton on:click=handle_delete />
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

        on_add(Ingredient {id:0, name: value.trim().to_owned(), quantity: None, certainty: None });

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
                    // TODO(filip): make dropwodn menu work
                    <button
                        type="button"
                        class="flex text-sm bg-gray-800 rounded-full md:me-0 focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600"
                        id="user-menu-button"
                        aria-expanded="false"
                        data-dropdown-toggle="user-dropdown"
                        data-dropdown-placement="bottom"
                    >
                        <span class="sr-only">"Open user menu"</span>
                        // <img class="w-8 h-8 rounded-full" src="" alt="user photo"/>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" class="w-8 h-8 rounded-full">
                          <circle cx="100" cy="70" r="50" fill="none" stroke="#f8f8f8" stroke-width="2" />
                          <path d="M50,70 L100,20 L150,70 Z" fill="#f8f8f8" stroke="#f8f8f8" stroke-width="2" />
                          <circle cx="80" cy="60" r="5" fill="black" />
                          <circle cx="120" cy="60" r="5" fill="black" />
                          <path d="M80,80 Q100,90 120,80" fill="#f8f8f8" stroke="#f8f8f8" />
                          <rect x="95" y="120" width="10" height="20" fill="#f8f8f8" stroke="#f8f8f8" />
                        </svg>
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
fn DeleteButton(
    #[prop(default = 5)]
    w: u16,
    #[prop(default = 5)]
    h: u16,
    #[prop(default = "button".to_owned())]
    btn_type: String
) -> impl IntoView {

    let size_class = "w-".to_owned()+&w.to_string()+" h-"+&h.to_string();

    view! {
        <button type={btn_type} class="text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-lg text-sm p-1.5 text-center inline-flex items-center dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900" >
            <svg class="w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 16">
              // <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z"/>
              // <path d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z"/>
<path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5M8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5m3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0"/>
            </svg>

            <span class="sr-only">"Delete ingredient button"</span> // TODO(filip): change aria text
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


#[component]
fn RecipeList(
    // recipes: ReadSignal<Vec<RecipeItem>>,
) -> impl IntoView {
    // TODO(filip): handle saving recipes
    // TODO(filip): handle rating recipes


    let (recipes, set_recipes) = create_signal(vec![RecipeItem { id: Uuid::new_v4(), text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed ultricies sed dui id mattis. Vivamus viverra consectetur mi, sit amet tincidunt diam facilisis et. Fusce id diam quis ex placerat maximus non ut nisi. Donec bibendum aliquet eros et hendrerit. Etiam scelerisque, ante ac hendrerit sollicitudin, sem orci ullamcorper erat, quis vestibulum leo ex et urna. Phasellus vulputate condimentum nisl ut elementum. Mauris vitae lacinia dolor. Aliquam risus nibh, iaculis id ultricies non, ultrices vitae arcu. Sed consequat maximus ultricies. Vivamus elementum sit amet est nec gravida. Vestibulum dignissim dolor velit, id imperdiet velit sodales et. Nulla sit amet maximus lorem. Integer aliquam, leo quis fermentum hendrerit, erat turpis venenatis lectus, non vestibulum tortor est ut erat. Fusce fermentum felis tincidunt, facilisis leo a, eleifend nisl. Mauris a felis at mi suscipit sagittis sed et tellus. Vestibulum ultricies orci quis odio blandit, at aliquet turpis finibus.

Quisque eget tempus urna. Sed laoreet metus massa. Donec dapibus quam et aliquam lacinia. Etiam purus enim, ultrices in augue a, dignissim condimentum lacus. Morbi pulvinar tempor arcu, sed mollis nisl rhoncus et. Donec fermentum at enim ut efficitur. Proin id pharetra lorem. Quisque vel massa sapien. In metus diam, suscipit sed quam a, accumsan interdum lorem. Praesent efficitur justo eget lacinia varius. Mauris tellus mi, cursus ac vehicula et, sagittis vel tellus. Curabitur imperdiet enim suscipit ullamcorper tristique. Integer ullamcorper erat quis dolor consectetur, at mollis tellus gravida. Sed venenatis leo dui, et tempor massa semper et. Proin posuere mollis massa a porta. Nunc sem dolor, commodo at turpis a, pharetra imperdiet dolor.

Maecenas pharetra diam et nulla accumsan fringilla. Vestibulum ut urna mauris. Vivamus eu sem dui. Duis placerat mi rhoncus ante rhoncus, id lacinia odio egestas. Mauris interdum posuere felis, et aliquet nisl tincidunt non. Curabitur at porttitor quam. Nulla at felis a dolor pharetra feugiat. Donec rhoncus risus neque, et rhoncus dolor imperdiet ac".to_owned() }]);

    view! {

        <div class="w-full p-2 bg-white border border-gray-200 rounded-lg shadow md:p-4 dark:bg-gray-800 dark:border-gray-700">
            <For
                each=recipes
                key=|r| r.id
                let:child
            >
                <Recipe recipe=child />
            </For>
        </div>
    }
}

#[component]
fn Recipe(
    recipe: RecipeItem,
) -> impl IntoView {
    view! {
        <div class="text-white" >
            {recipe.text}
        </div>
    }
}
