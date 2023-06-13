# Snake_Game_WASM
Using Rust to do this WASM little demo

Install the packages:
```sh
npm i
```

To get the wasm pkg:
```sh
wasm-pack build --target web
```

You may need to change the import path in `wasm_snake_game.js` from `import { random } from 'www/utils/random.js';` 
into `import { random } from '../www/utils/random.js';`

Finally run it:
```sh
npm run dev
```
