use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Copy)]
pub enum TimeRange {
    Week,
    Month,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    All,
    Done,
}

impl Default for TimeRange {
    fn default() -> Self {
        TimeRange::Week
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::All
    }
}

#[derive(Debug)]
pub struct DataRange {
    pub time: TimeRange,
    pub status: Status,
}

impl Default for DataRange {
    fn default() -> Self {
        Self {
            time: TimeRange::default(),
            status: Status::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn filter_by_range(&self, range: DataRange) -> Vec<&Todo> {
        let now = Utc::now();

        let time_start = match range.time {
            TimeRange::Week => {
                let weekday = now.weekday().num_days_from_monday() as i64;
                now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc() - Duration::days(weekday)
            }
            TimeRange::Month => now
                .date_naive()
                .with_day(1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc(),
        };

        self.todos
            .iter()
            .filter(|todo| {
                // 时间条件
                if todo.created_at < time_start {
                    return false;
                }

                // 状态条件
                match range.status {
                    Status::All => true,
                    Status::Done => todo.completed,
                }
            })
            .collect()
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let list = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        Ok(list)
    }

    /// 保存到文件
    pub fn save_to_file(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("序列化失败");
        fs::write(path, json).expect("写入文件失败");
    }

    /// 只读列出
    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }

    /// 添加 Todo
    pub fn add(&mut self, title: String) {
        let id = self.next_id();

        let todo = Todo {
            id,
            title,
            completed: false,
            created_at: Utc::now(),
        };

        println!("✓ 已添加 Todo: {}", todo.title);
        self.todos.push(todo);
    }

    /// 标记完成
    pub fn done(&mut self, id: u32) -> bool {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.completed = true;
                println!("✓ Todo #{} 已完成", id);
                return true;
            }
        }
        println!("✗ 未找到 Todo #{}", id);
        false
    }

    /// 删除
    pub fn del(&mut self, id: u32) -> bool {
        let before = self.todos.len();
        self.todos.retain(|t| t.id != id);

        if before != self.todos.len() {
            println!("✓ Todo #{} 已删除", id);
            true
        } else {
            println!("✗ 未找到 Todo #{}", id);
            false
        }
    }

    fn next_id(&self) -> u32 {
        self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }
}
