use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

fn reorder(input: &str) -> String {
    lazy_static! {
        static ref RE_CHS: Regex = Regex::new(r"[\u4e00-\u9fa5]+").unwrap();
        static ref RE_OTH: Regex = Regex::new(r"[^\u4e00-\u9fa5]+").unwrap();
    }
    let mut bt = BTreeMap::new();
    for input in RE_CHS.find_iter(input) {
        bt.insert(input.start(), (true, input.as_str()));
    }
    for input in RE_OTH.find_iter(input) {
        bt.insert(input.start(), (false, input.as_str()));
    }
    let mut out = String::new();
    for item in bt {
        match item.1 {
            (true, content) => {
                for slice in content.chars().collect::<Vec<_>>().chunks(3) {
                    if slice.len() == 3 {
                        out.push(slice[1]);
                        out.push(slice[0]);
                        out.push(slice[2]);
                    } else {
                        for ch in slice {
                            out.push(*ch);
                        }
                    }
                }
            }
            (false, content) => {
                out.push_str(content);
            }
        }
    }
    out
}
struct Reorder {
    link: ComponentLink<Self>,
    value: String,
}

enum Msg {
    UpdateValue(String),
}

impl Component for Reorder {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Reorder {
            link,
            value: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(input) => {
                self.value = input;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <textarea rows=20 cols=80
                        value=&self.value
                        autofocus=true
                        oninput=self.link.callback(|e: InputData| Msg::UpdateValue(e.value))
                        placeholder="在这里输入文字将会自动乱序显示在下面">
                    </textarea>
                </div>
                <div>
                    <textarea rows=20 cols=80
                        readonly=true
                        value=&reorder(&self.value)
                        placeholder="这里将会显示乱序文字">
                    </textarea>
                    //{&reorder(&self.value)}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Reorder>();
}
