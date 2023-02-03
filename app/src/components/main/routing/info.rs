use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::main::routing::Route;

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <>
            <h1 class="text-2xl flex justify-center">{ "Nápověda" }</h1>
            <article class="mx-10 mt-4 space-y-6">
                <section>
                    <h2 class="text-lg ml-10 mb-2">{ "Výběr balíčku slovíček" }</h2>
                    {"Seznam všech balíčků slovíček najdete "}
                    <Link<Route> to={Route::Index}>
                        <span class="underline decoration-2">
                            {"zde"}
                        </span>
                    </Link<Route>>
                    {"."}
                </section>
                <section class="space-y-2">
                    <h2 class="text-lg ml-10">{ "Procvičování slovíček" }</h2>
                    <p>
                    {"
                        Po vybrání balíčku se vám zobrazí několik textových políček.
                        Každé políčko představuje separátní tvar slova.
                    "}
                    </p>
                    <p>
                    {"
                        Pokud slovo vyplníte správně, zbarví se zeleně a váš kurzor automaticky přeskočí na další.
                        Pokud vyplníte všechna slova správně, automaticky se zobrazí nové slovo na procvičení.
                    "}
                    </p>
                    <p>
                    {"
                        Pokud si nevíte rady, zmáčkněte tlačítko `Prozradit`. Textová pole se automaticky vyplní a druhým stiskem tlačítka se posunete na další slovo.
                    "}
                    </p>
                    <p>
                    {"
                        Pokud slovíčko znáte, stačí zmáčknout tlačítko `Další` a automaticky se přesunete na nové slovíčko.
                    "}
                    </p>
                </section>
            </article>
        </>
    }
}