use dioxus::prelude::*;
use dioxus::desktop::*;
use dioxus::desktop::tao::event::Event;
use std::sync::mpsc::Receiver;

use super::json::*;
use super::utility::*;
use super::work::*;
use super::ui::*;

#[derive(Debug, Clone)]
pub struct App{
    pub json: Json,
    pub gui: Gui,
    pub file: File,
    pub backup: Backup,
}
impl Default for App{
    fn default() -> App{
        App { 
            json: Json::new(), 
            gui: Gui::default(),
            file: File::default(),
            backup: Backup::default()
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct Gui{
    pub result_message: String,
    pub start_sec: i64,
    pub is_show_dialog: bool,
}

pub fn use_effect_backup(//バックアップのマルチスレッド処理
    mut app: Signal<App>, 
    mut rxs: Signal<Vec<Receiver<Result<u64, std::io::Error>>>>
){
    if app.read().backup.state != WorkState::Busy{return;}
    let mut end_nums = Vec::new();
    for (u, b) in rxs.read().iter().enumerate(){
        match b.try_recv(){
            Ok(_)=>{
                app.write().backup.done_num += 1;
                end_nums.push(u);
            },
            _ =>{}
        }
    }
    if !end_nums.is_empty(){//終了した非同期処理を削除
        end_nums.reverse();
        for u in end_nums{ rxs.write().remove(u); }
    }
    if app.read().backup.state == WorkState::Busy && app.read().backup.done_num == app.read().backup.all_num{//非同期処理が終了した場合
        app.write().backup.state = WorkState::Done;
        let msg = String::from(&format!("バックアップが終了しました。経過時間: {}", get_time_string(app.read().gui.start_sec)));
        app.write().gui.result_message = msg;
        return;
    }
    let msg = format!("バックアップ中...({}/{}) {}", 
        app.read().backup.done_num, app.read().backup.all_num, 
        get_percent(app.read().backup.done_num, 
        app.read().backup.all_num)
    );
    let completed = (app.read().backup.done_num as f32 / app.read().backup.all_num as f32 * 100.0) as u64;
    let _ = set_progress_value(completed);
    app.write().gui.result_message = msg;

    for (u, b) in app().backup.bu_files.iter().enumerate(){
        if b.is_done {continue;}
        if rxs.read().len() > num_cpus::get() {continue;}
        if app.read().backup.assign_num == app.read().backup.all_num{continue;}
        let (tx, rx) = std::sync::mpsc::channel();
        let mut tmp = b.clone();
        std::thread::spawn(move || {
            let res = tmp.set_copy();
            tx.send(res).unwrap();
        });
        app.write().backup.assign_num += 1;
        rxs.push(rx);
        app.write().backup.bu_files[u].is_done = true; 
    }
}

pub fn event_handler<UserWindowEvent>(event: &Event<UserWindowEvent>, app: &mut Signal<App>){
    if let Event::WindowEvent{//ウィンドウサイズ変更時の処理
        event: WindowEvent::Resized(size),
        ..
    } = event {
        app.write().json.wi.width = size.width;
        app.write().json.wi.height = size.height;
    }
    if let Event::WindowEvent{//ウィンドウ位置変更時の処理
        event: WindowEvent::Moved(pos),
        ..
    } = event {
        app.write().json.wi.pos_x = pos.x;
        app.write().json.wi.pos_y = pos.y;
    }
    if let Event::WindowEvent{//exe終了時に情報保存する処理
        event: WindowEvent::CloseRequested, 
        ..
    } = event {
        app.read().json.save();
    }  
}