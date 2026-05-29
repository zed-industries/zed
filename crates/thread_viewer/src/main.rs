use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3003".into());
    let db_path = env::var("DB_PATH")
        .unwrap_or_else(|_| r"C:\Users\Lepip\AppData\Local\Zed\threads\threads.db".into());

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).expect("bind");
    println!("http://127.0.0.1:{port}");

    let db = Arc::new(db_path);

    for stream in listener.incoming() {
        let db = db.clone();
        std::thread::spawn(move || handle(stream.unwrap(), &db));
    }
}

fn handle(mut stream: TcpStream, db_path: &str) {
    let mut buf = [0; 4096];
    let _ = stream.read(&mut buf);
    let req = String::from_utf8_lossy(&buf);

    let (method, path) = parse_request(&req);

    match (method, path.as_str()) {
        ("GET", "/") | ("GET", "/threads") => serve_page(&mut stream, db_path, Page::Threads),
        ("GET", "/meta") => serve_page(&mut stream, db_path, Page::Metadata),
        ("GET", "/test") => serve_page(&mut stream, db_path, Page::Test),
        ("POST", p) if p.starts_with("/api/") => handle_api(&mut stream, db_path, p, &req),
        _ => {
            let html = format!("<h1>404</h1><a href='/'>Back</a>");
            respond(&mut stream, 404, &html);
        }
    }
}

fn parse_request(req: &str) -> (&str, String) {
    let first_line = req.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/").to_string();
    (method, path)
}

#[derive(PartialEq)]
enum Page {
    Threads,
    Metadata,
    Test,
}

fn serve_page(stream: &mut TcpStream, db_path: &str, page: Page) {
    let mut html = String::from(
        r#"<!DOCTYPE html><html><head><meta charset=utf-8>
<title>Zed Agent Control</title>
<style>
*{box-sizing:border-box;margin:0;padding:0}
body{font-family:system-ui;background:#1e1e2e;color:#cdd6f4;display:flex;min-height:100vh}
nav{width:200px;background:#181825;padding:20px 0;flex-shrink:0}
nav a{display:block;padding:10px 20px;color:#cdd6f4;text-decoration:none;font-size:14px}
nav a:hover,nav a.active{background:#313244;color:#cba6f7}
main{flex:1;padding:20px;overflow:auto}
h1{color:#f5c2e7;margin-bottom:16px}h2{color:#cba6f7;margin:20px 0 12px}
table{border-collapse:collapse;width:100%;margin:8px 0;font-size:13px}
th,td{padding:6px 10px;text-align:left;border:1px solid #45475a}
th{background:#313244;position:sticky;top:0}tr:hover{background:#313244}
.null{color:#585b70}code{background:#313244;padding:1px 4px;border-radius:3px}

.form-group{margin:12px 0}
label{display:block;font-size:13px;color:#a6adc8;margin-bottom:4px}
input,textarea,select{width:100%;padding:8px 10px;background:#313244;border:1px solid #45475a;
color:#cdd6f4;border-radius:4px;font-size:13px;font-family:inherit}
textarea{resize:vertical;min-height:80px}
button{padding:8px 16px;background:#cba6f7;color:#1e1e2e;border:none;border-radius:4px;
cursor:pointer;font-weight:600;font-size:13px;margin-right:8px;margin-top:8px}
button:hover{background:#b4befe}
button.danger{background:#f38ba8}
.result{margin-top:12px;padding:12px;background:#313244;border-radius:4px;font-size:13px;white-space:pre-wrap;max-height:300px;overflow:auto}
.status{display:inline-block;padding:2px 8px;border-radius:3px;font-size:12px;font-weight:600}
.status.ok{background:#a6e3a1;color:#1e1e2e}
.status.err{background:#f38ba8;color:#1e1e2e}
</style></head><body>
<nav>
<div style="padding:10px 20px;font-weight:700;color:#cba6f7;font-size:15px">Zed Control</div>
<a href="/threads" class=")"#,
    );

    html.push_str(if page == Page::Threads { "active" } else { "" });
    html.push_str(
        r#"">Threads DB</a>
<a href="/meta" class=")"#,
    );
    html.push_str(if page == Page::Metadata { "active" } else { "" });
    html.push_str(
        r#"">Metadata DB</a>
<a href="/test" class=")"#,
    );
    html.push_str(if page == Page::Test { "active" } else { "" });
    html.push_str(
        r#"">Test Agents</a>
</nav>
<main>"#,
    );

    match page {
        Page::Threads => html.push_str(&render_table(db_path, "threads", true)),
        Page::Metadata => html.push_str(&render_table(db_path, "sidebar_threads", false)),
        Page::Test => html.push_str(TEST_PAGE),
    }

    html.push_str("</main></body></html>");
    respond(stream, 200, &html);
}

fn render_table(db_path: &str, table: &str, use_threads_db: bool) -> String {
    let actual_db = if use_threads_db {
        db_path
    } else {
        "C:/Users/Lepip/AppData/Local/Zed/db/0-stable/db.sqlite"
    };

    let conn = match rusqlite::Connection::open_with_flags(
        actual_db,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    ) {
        Ok(c) => c,
        Err(e) => return format!("<p>Error: {e}</p>"),
    };

    let mut html = format!("<h1>{table}</h1><table><tr>");

    let columns: Vec<(String, String)> = match conn.prepare(&format!("PRAGMA table_info({table})"))
    {
        Ok(mut stmt) => stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect(),
        Err(_) => return "</table><p>Table not found</p>".into(),
    };

    for (name, _) in &columns {
        html.push_str(&format!("<th>{name}</th>"));
    }
    html.push_str("</tr>");

    let sql = if use_threads_db {
        format!("SELECT * FROM {table} ORDER BY rowid DESC LIMIT 50")
    } else {
        format!(
            "SELECT thread_id, session_id, agent_id, title, updated_at, created_at, folder_paths, archived FROM {table} ORDER BY updated_at DESC LIMIT 50"
        )
    };

    if let Ok(mut stmt) = conn.prepare(&sql) {
        let col_count = stmt.column_count();
        let mut rows = stmt.query([]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            html.push_str("<tr>");
            for i in 0..col_count {
                let val: String = match row.get_ref(i) {
                    Ok(v) => match v {
                        rusqlite::types::ValueRef::Null => "<span class=null>NULL</span>".into(),
                        rusqlite::types::ValueRef::Integer(n) => n.to_string(),
                        rusqlite::types::ValueRef::Real(f) => f.to_string(),
                        rusqlite::types::ValueRef::Text(t) => {
                            let s = String::from_utf8_lossy(t);
                            if s.len() > 200 {
                                format!("{}...", &s[..200])
                            } else {
                                s.into_owned()
                            }
                        }
                        rusqlite::types::ValueRef::Blob(b) => format!("[blob {}B]", b.len()),
                    },
                    Err(_) => "?".into(),
                };
                html.push_str(&format!("<td>{val}</td>"));
            }
            html.push_str("</tr>");
        }
    }
    html.push_str("</table>");
    html
}

const TEST_PAGE: &str = r#"
<h1>Test Agents</h1>
<p style="margin-bottom:16px;color:#a6adc8">Create and prompt agents via the HTTP API (default: <code>http://127.0.0.1:8765</code>).</p>

<div style="display:grid;grid-template-columns:1fr 1fr;gap:20px">
<div>
<h2>Create Agent</h2>
<div class="form-group"><label>Workdir</label><input id="create-workdir" placeholder="/path/to/project"></div>
<div class="form-group"><label>Model</label><input id="create-model" placeholder="use default"></div>
<div class="form-group"><label>Title</label><input id="create-title" placeholder="Agent title"></div>
<button onclick="createAgent()">Create</button>
<div class="result" id="create-result" style="display:none"></div>
</div>

<div>
<h2>Prompt Agent</h2>
<div class="form-group"><label>Session ID</label><input id="prompt-session" placeholder="uuid-from-create"></div>
<div class="form-group"><label>Prompt</label><textarea id="prompt-text" placeholder="Enter your prompt..."></textarea></div>
<button onclick="promptAgent()">Send</button>
<button onclick="getStatus()" style="background:#89b4fa">Status</button>
<button onclick="deleteAgent()" class="danger">Delete</button>
<div class="result" id="prompt-result" style="display:none"></div>
</div>
</div>

<script>
const API = 'http://127.0.0.1:8765';

async function createAgent() {
    const body = {};
    const wd = document.getElementById('create-workdir').value;
    const model = document.getElementById('create-model').value;
    const title = document.getElementById('create-title').value;
    if (wd) body.workdir = wd;
    if (model) body.model = model;
    if (title) body.title = title;

    const r = document.getElementById('create-result');
    r.style.display = 'block';
    r.innerHTML = '<span class="status ok">Sending...</span>';
    try {
        const res = await fetch(API + '/agents', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(body)
        });
        const data = await res.json();
        if (res.ok) {
            r.innerHTML = '<span class="status ok">Created</span>\nSession: ' + data.session_id + '\nModel: ' + data.model + '\nWorkdir: ' + data.workdir;
            document.getElementById('prompt-session').value = data.session_id;
        } else {
            r.innerHTML = '<span class="status err">Error</span>\n' + JSON.stringify(data, null, 2);
        }
    } catch(e) {
        r.innerHTML = '<span class="status err">Error</span>\n' + e.message;
    }
}

async function promptAgent() {
    const session = document.getElementById('prompt-session').value;
    const prompt = document.getElementById('prompt-text').value;
    if (!session || !prompt) return;

    const r = document.getElementById('prompt-result');
    r.style.display = 'block';
    r.innerHTML = '<span class="status ok">Running...</span>';
    try {
        const res = await fetch(API + '/agents/' + session + '/prompt', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({prompt})
        });
        const data = await res.json();
        if (res.ok) {
            r.innerHTML = '<span class="status ok">Done</span>\nStop: ' + data.stop_reason + '\nInput: ' + (data.input_tokens || 0) + ' tokens\nOutput: ' + (data.output_tokens || 0) + ' tokens';
        } else {
            r.innerHTML = '<span class="status err">Error</span>\n' + JSON.stringify(data, null, 2);
        }
    } catch(e) {
        r.innerHTML = '<span class="status err">Error</span>\n' + e.message;
    }
}

async function getStatus() {
    const session = document.getElementById('prompt-session').value;
    if (!session) return;

    const r = document.getElementById('prompt-result');
    r.style.display = 'block';
    try {
        const res = await fetch(API + '/agents/' + session);
        const data = await res.json();
        r.innerHTML = '<span class="status ok">Status</span>\n' + JSON.stringify(data, null, 2);
    } catch(e) {
        r.innerHTML = '<span class="status err">Error</span>\n' + e.message;
    }
}

async function deleteAgent() {
    const session = document.getElementById('prompt-session').value;
    if (!session) return;

    const r = document.getElementById('prompt-result');
    r.style.display = 'block';
    try {
        const res = await fetch(API + '/agents/' + session, {method: 'DELETE'});
        const data = await res.json();
        r.innerHTML = '<span class="status ok">Deleted</span>\n' + JSON.stringify(data, null, 2);
    } catch(e) {
        r.innerHTML = '<span class="status err">Error</span>\n' + e.message;
    }
}
</script>
"#;

fn handle_api(stream: &mut TcpStream, _db_path: &str, path: &str, req: &str) {
    let agent_path = path.strip_prefix("/api").unwrap_or(path);
    let agent_url = format!("http://127.0.0.1:8765{agent_path}");

    let first_line = req.lines().next().unwrap_or("");
    let method = first_line.split_whitespace().next().unwrap_or("GET");

    // Extract body: find \r\n\r\n, take JSON between { and }
    let body = if let Some(pos) = req.find("\r\n\r\n") {
        let raw = &req[pos + 4..];
        if let Some(json_start) = raw.find('{') {
            if let Some(json_end) = raw[json_start..].rfind('}') {
                raw[json_start..json_start + json_end + 1].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let client = reqwest::blocking::Client::new();
    let http_method = match method {
        "DELETE" => reqwest::Method::DELETE,
        "POST" => reqwest::Method::POST,
        _ => reqwest::Method::GET,
    };

    let mut http_req = client.request(http_method.clone(), &agent_url);
    if http_method == reqwest::Method::POST && !body.is_empty() {
        http_req = http_req
            .header("Content-Type", "application/json")
            .body(body);
    }

    match http_req.send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let text = resp.text().unwrap_or_default();
            let res = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{text}",
                text.len()
            );
            let _ = stream.write_all(res.as_bytes());
        }
        Err(e) => {
            let err = format!("{{\"error\":\"{e}\"}}");
            let res = format!(
                "HTTP/1.1 502\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{err}",
                err.len()
            );
            let _ = stream.write_all(res.as_bytes());
        }
    }
}

fn respond(stream: &mut TcpStream, status: u16, html: &str) {
    let resp = format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{html}",
        html.len()
    );
    let _ = stream.write_all(resp.as_bytes());
}
