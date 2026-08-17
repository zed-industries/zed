let l = []
e Cargo.lock
let id = jobstart('target/release/examples/bench_tokio', { 'rpc': v:true })

let start = reltime()
call rpcrequest(id, 'file')
let seconds = reltimestr(reltime(start))
call add(l, 'File Tokio: ' . seconds)

let start = reltime()
call rpcrequest(id, 'buffer')
let seconds = reltimestr(reltime(start))
call add(l, 'Buffer Tokio: ' . seconds)

let start = reltime()
call rpcrequest(id, 'api')
let seconds = reltimestr(reltime(start))
call add(l, 'API Tokio: ' . seconds)


let id = jobstart('target/release/examples/bench_async-std', { 'rpc': v:true })

let start = reltime()
call rpcrequest(id, 'file')
let seconds = reltimestr(reltime(start))
call add(l, 'File Async-Std: ' . seconds)

let start = reltime()
call rpcrequest(id, 'buffer')
let seconds = reltimestr(reltime(start))
call add(l, 'Buffer Async-Std: ' . seconds)

let start = reltime()
call rpcrequest(id, 'api')
let seconds = reltimestr(reltime(start))
call add(l, 'API Async-Std: ' . seconds)


let id = jobstart('target/release/examples/bench_sync', { 'rpc': v:true })

let g:started_file = reltime()
call rpcnotify(id, 'file')

call rpcnotify(id, 'buffer')

call rpcnotify(id, 'api')

sleep 20

call add(l, 'File Neovim-Lib: ' . g:finished_file)
call add(l, 'Buffer Neovim-Lib: ' . g:finished_buffer)
call add(l, 'API Neovim-Lib: ' . g:finished_api)

call nvim_buf_set_lines(0, 0, -1, v:false, l)
