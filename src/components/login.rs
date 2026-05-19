use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="flex w-screen min-h-screen bg-[#101525] text-slate-100">
            <div class="w-full grid grid-cols-1 lg:grid-cols-[1fr_420px]">
                <div class="relative flex flex-col justify-center px-8 py-10 sm:px-14 overflow-hidden bg-[radial-gradient(circle_at_25%_20%,_rgba(34,211,238,0.22),_transparent_28%),radial-gradient(circle_at_75%_70%,_rgba(217,70,239,0.2),_transparent_25%),linear-gradient(135deg,_#111827,_#171b2f)]">
                    <div class="absolute inset-x-0 bottom-0 h-24 border-t border-cyan-300/20 bg-[linear-gradient(90deg,_rgba(34,211,238,0.12)_1px,_transparent_1px),linear-gradient(0deg,_rgba(34,211,238,0.12)_1px,_transparent_1px)] bg-[size:32px_32px]"></div>
                    <div class="relative max-w-3xl">
                        <div class="mb-5 inline-flex border border-emerald-300/40 bg-emerald-400/10 px-3 py-1 text-xs font-black uppercase tracking-[0.24em] text-emerald-200">
                            {"Live lobby"}
                        </div>
                        <h1 class="text-5xl sm:text-7xl font-black leading-none text-white">
                            {"Yew Arena Chat"}
                        </h1>
                        <div class="mt-6 grid max-w-lg grid-cols-3 gap-3">
                            <div class="border border-cyan-300/20 bg-cyan-300/10 p-3 text-center">
                                <div class="text-2xl">{"⚔️"}</div>
                                <div class="mt-1 text-xs font-bold text-cyan-100">{"React"}</div>
                            </div>
                            <div class="border border-fuchsia-300/20 bg-fuchsia-300/10 p-3 text-center">
                                <div class="text-2xl">{"💬"}</div>
                                <div class="mt-1 text-xs font-bold text-fuchsia-100">{"Chat"}</div>
                            </div>
                            <div class="border border-amber-300/20 bg-amber-300/10 p-3 text-center">
                                <div class="text-2xl">{"🔥"}</div>
                                <div class="mt-1 text-xs font-bold text-amber-100">{"Combo"}</div>
                            </div>
                        </div>
                    </div>
                </div>
                <form class="flex flex-col justify-center gap-4 border-l border-cyan-300/20 bg-[#171b2f] p-8">
                    <div>
                        <div class="text-xs uppercase tracking-[0.24em] text-cyan-200">{"Player"}</div>
                        <div class="text-2xl font-black text-white">{"Choose name"}</div>
                    </div>
                    <input {oninput} class="w-full border border-cyan-300/25 bg-[#101525] p-4 text-slate-100 outline-none focus:border-cyan-200" placeholder="Username" />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="w-full border border-fuchsia-200/40 bg-fuchsia-600 p-4 font-black uppercase text-white hover:bg-fuchsia-500 disabled:cursor-not-allowed disabled:border-slate-500/30 disabled:bg-slate-700 disabled:text-slate-400">
                            {"Enter Lobby"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
