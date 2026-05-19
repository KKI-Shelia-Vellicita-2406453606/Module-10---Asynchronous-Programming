use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    React { message_id: String, emoji: String },
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageData {
    #[serde(default)]
    id: String,
    from: String,
    message: String,
    #[serde(default)]
    time: u64,
    #[serde(default)]
    reactions: HashMap<String, u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReactionUpdate {
    message_id: String,
    emoji: String,
    count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
    Reaction,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    current_user: String,
}

fn avatar_url(name: &str) -> String {
    format!(
        "https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}",
        name
    )
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            current_user: username,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                                .map(|u| UserProfile {
                                    name: u.into(),
                                    avatar: avatar_url(u),
                                })
                                .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    MsgTypes::Reaction => {
                        let reaction: ReactionUpdate =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        if let Some(message) = self
                            .messages
                            .iter_mut()
                            .find(|message| message.id == reaction.message_id)
                        {
                            message.reactions.insert(reaction.emoji, reaction.count);
                            return true;
                        }
                        return false;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    if value.trim().is_empty() {
                        return false;
                    }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(value),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
            Msg::React { message_id, emoji } => {
                let payload = serde_json::json!({
                    "messageId": message_id,
                    "emoji": emoji,
                });
                let message = WebSocketMessage {
                    message_type: MsgTypes::Reaction,
                    data: Some(payload.to_string()),
                    data_array: None,
                };

                if let Err(e) = self
                    .wss
                    .tx
                    .clone()
                    .try_send(serde_json::to_string(&message).unwrap())
                {
                    log::debug!("error sending reaction to channel: {:?}", e);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let reaction_emojis = ["⚔️", "✨", "🔥"];
        let link = ctx.link().clone();

        html! {
            <div class="flex w-screen h-screen bg-[#121524] text-slate-100">
                <div class="flex-none w-64 h-screen border-r border-cyan-300/20 bg-[#171b2f]">
                    <div class="p-4 border-b border-cyan-300/20">
                        <div class="text-xs uppercase tracking-[0.28em] text-cyan-200">{"Party"}</div>
                        <div class="text-2xl font-black text-white">{"Yew Arena"}</div>
                    </div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 border border-cyan-300/20 bg-[#202842] p-2 shadow-[0_0_0_2px_rgba(255,255,255,0.04)]">
                                    <div>
                                        <img class="w-12 h-12 rounded-sm bg-cyan-100" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div class="font-bold text-cyan-100">{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-emerald-300">
                                            {"online"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-16 border-b border-cyan-300/20 bg-[#181f35] flex items-center justify-between px-5">
                        <div>
                            <div class="text-xs uppercase tracking-[0.24em] text-fuchsia-200">{"Guild chat"}</div>
                            <div class="text-lg font-black">{"Quest Board"}</div>
                        </div>
                        <div class="px-3 py-1 border border-emerald-300/40 bg-emerald-400/10 text-xs font-bold text-emerald-200">
                            {format!("{} players", self.users.len())}
                        </div>
                    </div>
                    <div class="w-full grow overflow-auto border-b border-cyan-300/20 bg-[radial-gradient(circle_at_top_right,_rgba(34,211,238,0.18),_transparent_28%),linear-gradient(180deg,_#111827,_#15162a)]">
                        {
                            self.messages.iter().map(|m| {
                                let avatar = self
                                    .users
                                    .iter()
                                    .find(|u| u.name == m.from)
                                    .map(|u| u.avatar.clone())
                                    .unwrap_or_else(|| avatar_url(&m.from));
                                let is_me = m.from == self.current_user;
                                let message_id = m.id.clone();
                                let bubble_side = if is_me { "ml-auto rounded-tl-lg" } else { "mr-auto rounded-tr-lg" };
                                let bubble_color = if is_me {
                                    "border-violet-300/30 bg-violet-500/20"
                                } else {
                                    "border-cyan-300/25 bg-[#202842]"
                                };
                                html!{
                                    <div class={classes!("flex", "items-end", "w-4/6", "max-w-2xl", "m-6", bubble_side)}>
                                        if !is_me {
                                            <img class="w-10 h-10 rounded-sm m-3 bg-cyan-100 border-2 border-cyan-200" src={avatar.clone()} alt="avatar"/>
                                        }
                                        <div class={classes!("grow", "border", "p-4", "shadow-lg", bubble_color)}>
                                            <div class="flex items-center justify-between gap-3">
                                                <div class="text-sm font-black text-white">
                                                    {m.from.clone()}
                                                </div>
                                                <div class="text-[10px] uppercase tracking-[0.2em] text-slate-400">
                                                    {format!("#{}", m.time % 100000)}
                                                </div>
                                            </div>
                                            <div class="mt-2 text-sm text-slate-200 break-words">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3 max-h-64 border border-cyan-300/20" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                            <div class="mt-3 flex flex-wrap items-center gap-2">
                                                {
                                                    reaction_emojis.iter().filter_map(|emoji| {
                                                        m.reactions.get(*emoji).map(|count| html! {
                                                            <span class="border border-amber-300/30 bg-amber-300/10 px-2 py-1 text-xs font-bold text-amber-100">
                                                                {format!("{} {}", emoji, count)}
                                                            </span>
                                                        })
                                                    }).collect::<Html>()
                                                }
                                                if !is_me && !message_id.is_empty() {
                                                    <>
                                                        {
                                                            reaction_emojis.iter().map(|emoji| {
                                                                let onreact = {
                                                                    let message_id = message_id.clone();
                                                                    let emoji = emoji.to_string();
                                                                    link.callback(move |_| Msg::React {
                                                                        message_id: message_id.clone(),
                                                                        emoji: emoji.clone(),
                                                                    })
                                                                };
                                                                html! {
                                                                    <button onclick={onreact} class="h-8 w-8 border border-cyan-300/30 bg-cyan-300/10 text-sm hover:bg-cyan-300/20" title="react">
                                                                        {emoji}
                                                                    </button>
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </>
                                                }
                                        </div>
                                    </div>
                                        if is_me {
                                            <img class="w-10 h-10 rounded-sm m-3 bg-violet-100 border-2 border-violet-200" src={avatar.clone()} alt="avatar"/>
                                        }
                                    </div>
                                }
                            }).collect::<Html>()
                        }

                    </div>
                    <div class="w-full h-20 flex px-4 items-center bg-[#181f35]">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Type a move..." class="block w-full py-3 pl-4 mx-3 border border-cyan-300/25 bg-[#101525] text-slate-100 outline-none focus:border-cyan-200" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-sm bg-fuchsia-600 w-12 h-12 flex justify-center items-center border border-fuchsia-200/40 hover:bg-fuchsia-500">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
