cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir .\website\out\ --target web .\target\wasm32-unknown-unknown\release\fcd-practice.wasm
Xcopy .\assets\ .\website\assets\ /e /s /y
py -m http.server 45744 --directory .\website\