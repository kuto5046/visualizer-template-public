# run the app
run:
  cd wasm && wasm-pack build --target web --out-dir ../public/wasm && cd .. && yarn dev
