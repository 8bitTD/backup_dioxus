pub fn get_percent(num: usize, all_num: usize) -> String{//〇〇〇%
    let val = num as f32 / all_num as f32;
    let u_val = (val * 100.0) as usize;
    format!("{:02}%", u_val)
}

pub fn get_now_sec() -> i64{//現在のタイムスタンプ(秒)を取得する処理
    return chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()).naive_local().and_utc().timestamp()
}

pub fn get_time_string(start_sec: i64) -> String{//〇〇時間〇〇分〇〇秒
    let end_sec = get_now_sec();
    let sec = end_sec - start_sec;
    let mut res_h: u64 = 0;
    let mut res_m: u64 = 0;
    let mut res_s: u64 = sec as u64;
    if res_s >= 3600{
        res_h = res_s / 3600;
        res_s = res_s % 3600;
    }
    if res_s >= 60{
        res_m = res_s / 60;
        res_s = res_s % 60;
    }
    if res_h > 0{
        let res = format!("{}{}{}{}{}{}", res_h," 時間 ",res_m," 分 ",res_s," 秒");
        return res;
    }else if res_m > 0{
        let res = format!("{}{}{}{}", res_m," 分 ",res_s," 秒");
        return res;
    }else{
        let res = format!("{}{}",res_s," 秒");
        return res;
    }
}
