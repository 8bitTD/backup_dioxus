use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum WorkState{
    #[default]
    Idle,
    Busy,
    Done,
}

#[derive(Debug, Clone, Default)]
pub struct File{
    pub state: WorkState,
    pub file_paste: String,
    pub file_copies: Vec<String>,
}

impl File{
    pub fn new(file_paste: &str, file_copies: &str) -> File{
        let file_paste = file_paste.to_string();
        let tmp:Vec<&str> = file_copies.split("\n").collect();
        let file_copies = tmp.iter().map(|&t|t.to_string()).collect();
        File { 
            state: WorkState::Idle,
            file_paste: file_paste, 
            file_copies: file_copies,
        }
    }
    pub fn get_incorrect_path(&self) -> String {//正しくないパスを取得する処理、正しい場合は空の文字列を返す
        if !std::path::Path::new(&self.file_paste).is_dir(){
            return self.file_paste.to_string();
        }
        for c in &self.file_copies{
            if !std::path::Path::new(&c).is_dir(){
                return c.to_string();
            }
        }
        return String::new();
    }
}

#[derive(Debug, Clone)]
pub struct FileResult{
    pub state: WorkState,
    pub message: String,
    pub files: Vec<BackupFile>
    
}
impl FileResult{
    pub fn new(state: WorkState, message: &str, files: Vec<BackupFile>) -> FileResult{
        FileResult{
            state: state,
            message: String::from(message),
            files: files,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Default)]
pub struct Backup{
    pub bu_files: Vec<BackupFile>,
    pub state: WorkState,
    pub all_num: usize,
    pub assign_num: usize,
    pub done_num: usize,
}
impl Backup{
    pub fn clear(&mut self){
        self.bu_files.clear();
        self.state = WorkState::Idle;
        self.all_num = 0;
        self.assign_num = 0;
        self.done_num = 0;
    }
}

#[derive(Debug, Clone)]
pub struct BackupFile{
    pub paste_file: String,
    pub copy_file: String,
    pub is_done: bool
}
impl BackupFile{
    pub fn new(paste_file: &str, copy_file: &str) -> BackupFile{
        BackupFile { 
            paste_file: paste_file.to_string(), 
            copy_file: copy_file.to_string(),
            is_done: false,
        }
    }

    pub fn set_copy(&mut self) -> Result<u64, std::io::Error>{//バックアップ処理を実行
        let oya = std::path::Path::new(&self.paste_file).parent().unwrap().to_str().unwrap().to_string();
        if !std::path::Path::new(&oya).is_dir(){Some(std::fs::create_dir_all(&oya));}
        let res = std::fs::copy(&self.copy_file,&self.paste_file);
        return res;
    }
}

pub fn get_files(tx: UnboundedSender<FileResult>, files: &mut File){
    let msg = String::from("ファイルを取得中...");
    let _ = tx.unbounded_send(FileResult::new(WorkState::Busy,&msg, Vec::new()));
    let mut resut_files = Vec::new();
    for (u, path) in files.file_copies.iter().enumerate(){
        if path.is_empty(){continue;}
        let copy_oya_folder = std::path::Path::new(&path).parent().unwrap().to_str().unwrap().to_string();
        for entry in walkdir::WalkDir::new(&path) {
            let entry = entry.unwrap();
            if !entry.file_type().is_file(){continue;}
            let copy_file = entry.path().display().to_string().replace("\\","/");
            let tmp_folder = std::path::Path::new(&copy_file).parent().unwrap().to_str().unwrap().to_string();
            let mut copy_folder = tmp_folder.replace(&copy_oya_folder, "");
            if copy_folder.chars().nth(0).unwrap().to_string() == String::from("/"){copy_folder.remove(0);}
            let file_name = std::path::Path::new(&copy_file).file_name().unwrap().to_str().unwrap().to_string();
            let paste_file = format!("{}/{:02}_{}/{}", files.file_paste, u+1, copy_folder, &file_name);
            let bu = BackupFile::new(&paste_file, &copy_file);
            resut_files.push(bu);
            let msg = format!("ファイルを取得中...{}", &resut_files.len());
            let _ = tx.unbounded_send(FileResult::new(WorkState::Busy,&msg, resut_files.clone()));
        }
    }
    let all_num = resut_files.len();
    let msg = String::from(&format!("{}ファイルを取得しました", all_num));
    let _ = tx.unbounded_send(FileResult::new(WorkState::Done,&msg, resut_files.clone()));
}