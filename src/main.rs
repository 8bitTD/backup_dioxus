#![allow(non_snake_case)]
//#![windows_subsystem = "windows"]
use dioxus::prelude::*;
use dioxus::desktop::muda::*;
use icon::load_icon_from_url;

mod app;
mod define;
mod icon;
mod json;
mod ui;
mod utility;
mod work;

fn main() {
    set_exec();
}

fn set_exec(){
    let mut json = json::Json::new();
    json.load();
    let pos = dioxus::desktop::tao::dpi::PhysicalPosition::new(json.wi.pos_x, json.wi.pos_y);
    let size = dioxus::desktop::tao::dpi::PhysicalSize::new(json.wi.width, json.wi.height);
    let ico = load_icon_from_url(define::common::ICONURL);
    let wb = dioxus::desktop::WindowBuilder::new()
        .with_always_on_top(false)
        .with_window_icon(ico)
        .with_title(define::common::TOOLNAME).with_position(pos).with_inner_size(size);
    let config = dioxus::desktop::Config::new()
        .with_menu(Menu::new())
        .with_custom_index(custom_html())
        .with_window(wb);
    LaunchBuilder::new().with_cfg(config).launch(ui::ui);
}

fn custom_html() -> String{
    r#"
        <!DOCTYPE html>
        <html>
            <head>

                <style>
                    :root {
                        font-family: Inter, Avenir, Meiryo, Helvetica, Arial, sans-serif;
                        /*font-family: "myfont";*/
                        font-size: 16px;
                        line-height: 24px;
                        font-weight: 400;
                        color: #0f0f0f;
                        background-color: #f6f6f6;
                        font-synthesis: none;
                        text-rendering: optimizeLegibility;
                        -webkit-font-smoothing: antialiased;
                        -moz-osx-font-smoothing: grayscale;
                        -webkit-text-size-adjust: 100%;
                    }

                    textarea {
                        resize: none;
                        color: #ffffff;
                        tab-size: 4; 
                        overflow:hidden;
                    }

                    input, button, textarea{
                        border-radius: 4px;
                        border: 1px solid transparent;
                        padding: 0.2em 0.4em;
                        color: #242424;
                        font-family: inherit;
                        background-color: #ffffff;
                        transition: border-color 0.25s;
                        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
                        margin: 1px;
                    }

                    hr {
                        height: 0;
                        margin: 0;
                        padding: 0;
                        border: 0;
                        border-top: 1px dotted #aaaaaa;
                    }

                    input:disabled, button:disabled, textarea:disabled{
                        opacity: 30%;
                    }

                    button:hover, textarea:hover, input:hover {
                        border-color: #396cd8;
                    }

                    img:hover{
                        filter: brightness(1.10);
                    }

                    img:active{
                        filter: brightness(0.5);
                    }

                    button:active {
                        border-color: #396cd8;
                        background-color: #e8e8e8;
                    }

                    @keyframes rotating {
                        from { transform: rotate(0deg); }
                        to { transform: rotate(360deg); }
                    }
                    .rotating {
                        animation: rotating 1s linear infinite;
                    }

                    @media (prefers-color-scheme: dark) {
                        :root {
                            color: #f6f6f6;
                            background-color: #2f2f2f;
                            color-scheme: dark;
                        }
                        a {
                            color:rgb(26, 156, 170);
                        }
                        a:hover {
                            color: #24c8db;
                        }
                        hr {
                            border-top: 1px dotted rgb(94, 94, 94);
                        }
                        input, textarea, dialog{
                            color:  #ffffff;
                            /*text-shadow: 0 0 2px  #529dff,0 0 4px #0066ff;*/
                            background-color: #202020;
                        }
                        dialog{
                            border-color:rgb(83, 83, 83);
                        }
                        button{
                            color: #ffffff;
                            background-color: #0f0f0f;
                        }
                        button:active {
                            background-color: #0f0f0f;
                        }
                        
                        .sidebar {
                            border: 1px #535353 solid;
                        }
                    }
                </style>
            </head>
            <body>
                <div id="main"></div>
            </body>
        </html>
    "#.into()
}