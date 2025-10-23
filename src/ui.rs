use dioxus::prelude::*;
use dioxus::desktop::use_wry_event_handler;
use futures_util::StreamExt;
use std::sync::mpsc::Receiver;
use winsafe::{self as w, prelude::*, co};

use crate::ui::w::ITaskbarList4;

use super::app::*;
use super::work::*;
use super::utility::*;
use super::define::*;

#[component]
pub fn ui() -> Element {
    let app = use_signal(|| App::default());
    let rx_backup = use_signal(|| { //バックアップ処理のマルチスレッド処理用のチャンネルの受信変数
        Vec::<Receiver<Result<u64, std::io::Error>>>::new()
    });
    use_effect(move || { use_effect_backup(app, rx_backup); });//バックアップのマルチスレッド処理
    main_ui(app, rx_backup)//メインUI
}

pub fn main_ui(
    mut app: Signal<App>, 
    mut rx_backup: Signal<Vec<Receiver<Result<u64, std::io::Error>>>>
) -> Element {
    let is_running = use_memo(move || {//処理中かどうかの判定変数
        app.read().file.state == WorkState::Busy || app.read().backup.state == WorkState::Busy 
    });
    use_wry_event_handler(move |event, _| { event_handler(event, &mut app); });//ウィンドウ変更時の記録用処理
    let rx_file = use_coroutine(move |mut rx: UnboundedReceiver<FileResult>| async move {
        while let Some(res) = rx.next().await { 
            app.write().gui.result_message = res.message;
            app.write().file.state = res.state;
            app.write().backup.bu_files = res.files;
            if app.read().file.state == WorkState::Done{
               let all_num = app.read().backup.bu_files.len();
               app.write().backup.all_num = all_num;
               app.write().backup.state = WorkState::Busy;
            }
        }
    });
    
    rsx!{
        div {
            style: "margin: 1px;",
            dialog { 
                style: "z-index: 10;",
                open: "{app.read().gui.is_show_dialog}",
                div {
                    style: "text-align: center;",
                    h3{"クレジット"}
                    hr {}
                    p {  
                        a { "このツールは " }
                        a {  
                            style: "width: 75%; height: 50%;",
                            href: "https://icons8.jp/icons",
                            "icons8.com"
                        },
                        a { " さんのアイコンを使用しております。" }
                    },
                    button { onclick: move|_| async move { app.write().gui.is_show_dialog = false; }, "閉じる" }
                }
            }
            input{
                style: "width: calc(100% - 75px);",
                r#type: "text",
                disabled: "{is_running.read()}",
                placeholder: "バックアップ先",
                oninput: move |ev| {
                    let res = ev.data().value();
                    app.write().json.backup_path = res.replace("\\","/");
                },
                value: "{app.read().json.backup_path}",
            }
            button{
                disabled: "{is_running.read()}",
                onclick: move |_| async move {
                    let res = native_dialog::DialogBuilder::file()
                        .set_location(&app.read().json.backup_path.to_string())
                        .open_single_dir()
                        .show().unwrap();
                    if res.is_some(){
                        app.write().json.backup_path = res.unwrap().to_str().unwrap().replace("\\","/"); 
                    }
                    app.write().gui.result_message = String::new();
                },
                "..."
            }
            button { 
                disabled: "{is_running.read()}",
                style: "position: relative; width: 26px; height: 26px; top: 6px;",
                title: "クレジット",
                onclick: move |_| { app.write().gui.is_show_dialog = !app().gui.is_show_dialog; }, 
                img {  
                    style:"position:relative; left: -4px; top: -1px;",
                    src: "https://img.icons8.com/color/20/terms-and-conditions.png",
                }
                ""
            }
        }
        div{ 
            style: "width: 100%; height: 25px; text-align: center; font-size: 15px; margin-top: -2px; margin-bottom: 5px;", 
            img{ 
                style: if is_running() {
                    "position:relative; left: 0px; top: 5px; margin-right: 5px;"
                }else{
                    "position:relative; left: 0px; top: 5px; margin-right: 5px; visibility: hidden;"
                },
                class: "rotating",
                src: "https://img.icons8.com/nolan/22/spinner-frame-3.png"
            }
            "{app.read().gui.result_message}" 
        }
        div{
            textarea { 
                style: "width: calc(100% - 13px); height: calc(100vh - 130px); resize: none;",
                disabled: "{is_running.read()}",
                placeholder: "バックアップ元",
                oninput: move |ev| {
                    let res = ev.data().value();
                    app.write().json.backup_text = res.replace("\\","/");
                },
                value: "{app.read().json.backup_text}",
            }
        }
        button {  
            style: "width: 100%;",
            disabled: "{is_running.read()}",
            onclick: move |_| async move{
                let bp = app.read().json.backup_path.to_string();
                let bt = app.read().json.backup_text.to_string();
                app.write().file = File::new(&bp, &bt);
                let res = app.read().file.get_incorrect_path();                
                if !res.is_empty(){
                    let msg = &format!("パスが存在しません！\n{}", res);
                    let _ = native_dialog::DialogBuilder::message()
                        .set_title("確認")
                        .set_level(native_dialog::MessageLevel::Warning)
                        .set_text(&msg)
                        .alert().show();
                }else{
                    app.write().gui.start_sec = get_now_sec();
                    app.write().backup.clear();
                    rx_backup.write().clear();
                    let tx = rx_file.tx();
                    let mut file_c = app.read().file.clone();
                    std::thread::spawn(move || { get_files(tx, &mut file_c) });
                }
                
            },
            "実行"
        }
    }
}

pub fn set_progress_value(completed: u64) -> winsafe::HrResult<()>{//タスクバーアイコンに進捗％を設定
    let op_hwnd = w::HWND::FindWindow(None, Some(common::TOOLNAME)).unwrap();
    let hwnd = op_hwnd.unwrap();
    let itbl: ITaskbarList4 = w::CoCreateInstance(
        &co::CLSID::TaskbarList,
        None::<&w::IUnknown>,
        co::CLSCTX::INPROC_SERVER,
    )?;
    itbl.SetProgressValue(&hwnd, completed, 100)?;
    Ok(())
}