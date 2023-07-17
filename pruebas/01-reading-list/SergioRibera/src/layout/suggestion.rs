use rand::{seq::SliceRandom, thread_rng};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::{models::Book as BookModel, route::Route};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub books: Vec<BookModel>,
}

#[function_component]
pub fn SuggestedBook(props: &Props) -> Html {
    let Props { books } = props.clone();
    let book = books.choose(&mut thread_rng());

    if let Some(book) = book {
        html! {
            <section class={classes!("flex","flex-row","flex-wrap","py-8","gap-8","justify-center","md:justify-evenly","items-center")}>
                <div
                    class={classes!("flex","flex-col","flex-wrap","gap-5","justify-start","max-w-[300px]")}
                >
                    <h1 class={classes!("text-neutral-600","font-bold","text-4xl")}>
                        {"Feliz Lectura"}
                    </h1>
                    <p class={classes!("text-neutral-600","font-bold")}>
                        {"¿Buscas una nueva aventura literaria? Deja que nuestros expertos te sugieran un libro perfecto para ti"}
                    </p>
                    <Link<Route>
                        to={Route::Book { name: book.title.clone() }}
                        classes={classes!("rounded-full","bg-slate-900","text-white","px-4","py-2","w-fit","flex","flex-row","gap-3")}
                    >
                        {"Comenzar a leer"}
                        <Icon icon_id={IconId::HeroiconsMiniSolidArrowUpRight} />
                    </Link<Route>>
                </div>
                    <Link<Route>
                        to={Route::Book { name: book.title.clone() }}
                        classes={classes!("flex","flex-row","book-image-shadow","transition-transform","hover:scale-105","cursor-pointer")}>
                    <img
                        class={classes!("w-[177px]","h-[266px]")}
                        src={book.cover.clone()}
                        alt={book.title.clone()}
                    />
                    <p class={classes!("flex","w-[177px]","h-[266px]","text-blur","items-center","justify-start","px-6")}>
                            {book.synopsis.clone()}
                    </p>
                    </Link<Route>>
                <div
                    class={classes!("flex","flex-col","flex-wrap","gap-5","justify-start","max-w-[300px]")}
                >
                    <div class={classes!("flex","flex-col")}>
                        <h1 class={classes!("text-neutral-600","font-bold","text-3xl")}>
                            {book.title.clone()}
                        </h1>
                        <p class={classes!("text-neutral-600","font-bold","float-right")}>
                            {book.author.name.clone()}
                        </p>
                    </div>
                    <div class={classes!("flex","flex-row","gap-3")}>
                        <span class={classes!("text-neutral-600","font-bold")}>
                            {"Paginas:"}
                        </span>
                        <span class={classes!("text-neutral-600","font-normal")}>
                            {book.pages}
                        </span>
                    </div>
                    <p class={classes!("text-neutral-600","font-normal")}>
                        {book.synopsis.clone()}
                    </p>
                </div>
            </section>
        }
    } else {
        html! {<></>}
    }
}
