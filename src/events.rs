
use crossterm::event::{KeyEvent, KeyCode};

use crate::{App, AppState};

pub fn on_events(event:KeyEvent, app:&mut App) {
    let code = event.code;
    let total = app.stocks.len(); 
    let sel = app.stocks_state.selected().unwrap_or(0);
    let selsome = app.stocks_state.selected().is_some();
    match app.state {
        AppState::Normal => {
            if code == KeyCode::Char('q') {
                app.should_exit = true;
            }
            else if code == KeyCode::Char('n') {
                //新建stock
                app.state = AppState::Adding;
                app.input = String::new();
            }
            else if code == KeyCode::Char('d') && selsome {
                //删除当前选中的stock
                app.stocks.remove(sel);
                app.save_stocks().unwrap();
                app.stocks_state.select(None);
            }
            else if code == KeyCode::Char('u') && selsome && sel > 0 {
                //将选中stock往上移动一位
                app.stocks.swap(sel, sel -1);
                app.save_stocks().unwrap();
                app.stocks_state.select(Some(sel - 1));
            }
            else if code == KeyCode::Char('j') && selsome && sel < total - 1 {
                //将选中stock往下移动一位
                app.stocks.swap(sel, sel + 1);
                app.save_stocks().unwrap();
                app.stocks_state.select(Some(sel + 1));
            }
            else if code == KeyCode::Up && total > 0 {
                //注意这里如果不加判断直接用sel - 1, 在sel为0时会导致异常
                app.stocks_state.select(Some(if sel > 0 {sel - 1} else {0}));
            }
            else if code == KeyCode::Down && total > 0 {
                app.stocks_state.select(Some(if sel < total - 1 {sel + 1} else {sel}));
            }
        },

        AppState::Adding => match code {
            KeyCode::Enter => {
                app.state = AppState::Normal;
            }
            KeyCode::Esc => {
                app.state = AppState::Normal;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            _ => {}
        },
    }
}