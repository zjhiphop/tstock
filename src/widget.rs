use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Dataset, GraphType, Chart, Axis, Wrap}, symbols,
};

use crate::{App, AppState, Stock};
use unicode_width::UnicodeWidthStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

//计算所有的屏幕窗口区域,供后续render使用
pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(parent[1]);

    let vcenter =  Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(center[1]);

    //计算新建stock时的弹框位置
    let popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(area);

    let popline = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(popup[1]);

    vec![parent[0], center[0], vcenter[0], parent[2], popline[1], vcenter[1]]
}

pub fn stock_list(stocks: &Vec<Stock>) -> List {
    let items: Vec<_> = stocks
        .iter()
        .map(|stock| {
            ListItem::new(Spans::from(vec![
                Span::styled(
                    format!("{:+.2}% ", stock.percent * 100.0),
                    Style::default().fg(if stock.percent < 0.0 {
                        Color::Green
                    } else {
                        Color::Red
                    }),
                ),
                Span::styled(stock.title.clone(), Style::default()),
            ]))
        })
        .collect();

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("列表")
                .border_type(BorderType::Plain),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
}

pub fn stock_detail(app: &App) -> Paragraph {
    let mut info = String::new();
    let sel = app.stocks_state.selected().unwrap_or(0);
    //这里要防止sel超出列表范围
    let stocks = app.stocks.lock().unwrap();
    if app.stocks_state.selected().is_some() && sel < stocks.len() {
        let stock = stocks.get(sel).unwrap();
        info = format!(
            "代码:{}\n涨跌:{:+.2}%\n当前:{}\n今开:{}\n昨收:{}\n最高:{}\n最低:{}",
            stock.code,
            stock.percent * 100.0,
            stock.price,
            stock.open,
            stock.yestclose,
            stock.high,
            stock.low
        );
    }

    Paragraph::new(info)
        .alignment(Alignment::Center)
        .style(Style::default())
        .block(
            Block::default()
                .title("详情")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )

        
    // Paragraph::new("test test test test test test test test test test test t test test test test test test t test test test test test test ")
    //     .block(Block::default().title("Paragraph").borders(Borders::ALL))
    //     .style(Style::default().fg(Color::White).bg(Color::Black))
    //     .alignment(Alignment::Center)
    //     .wrap(Wrap { trim: false })
}

pub fn stock_line_chart(app: &App) -> Chart {
    let datasets = vec![
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Cyan))
            .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];
    Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(Axis::default()
            .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()))
}

pub fn stock_input(app: &App) -> Paragraph {
    Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("输入证券代码"))
}

pub fn title_bar(app: &App, rect: Rect) -> Paragraph {
    let left = format!("Stock v{}", VERSION);
    let error = app.error.lock().unwrap();
    let right = if error.is_empty() {
        app.last_refresh
            .lock()
            .unwrap()
            .format("最后更新 %H:%M:%S")
            .to_string()
    } else {
        error.clone()
    };
    Paragraph::new(Spans::from(vec![
        Span::raw(left.clone()),
        //使用checked_sub防止溢出
        Span::raw(
            " ".repeat(
                (rect.width as usize)
                    .checked_sub(right.width() + left.width())
                    .unwrap_or(0),
            ),
        ),
        Span::styled(
            right,
            Style::default().fg(if error.is_empty() {
                Color::White
            } else {
                Color::Red
            }),
        ),
    ]))
    .alignment(Alignment::Left)
}

pub fn status_bar(app: &mut App) -> Paragraph {
    Paragraph::new(
        match app.state {
            AppState::Normal => "退出[Q] | 新建[N] | 删除[D] | 刷新[R] | 上移[U] | 下移[J]",
            AppState::Adding => "确认[Enter] | 取消[ESC] | 上交所代码前需要加0，深市加1",
        }
        .to_string(),
    )
    .alignment(Alignment::Left)
}
