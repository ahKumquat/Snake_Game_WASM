import __wbg_init, {Direction, GameStates, World} from "wasm_snake_game";
import {random} from "./utils/random";
__wbg_init().then(wasm => {
    const CELL_SIZE = 20
    const WORLD_WIDTH = 8
    const snakeIdx = random(WORLD_WIDTH * WORLD_WIDTH)
    const world = World.new(WORLD_WIDTH,snakeIdx);
    const worldWidth = world.get_width();
    const fps = 5;

    const canvas = <HTMLCanvasElement>document.getElementById("world");
    const gameStatus = document.getElementById("game-status");
    const gameControlBtn = document.getElementById("game-control-btn");
    const ctx = canvas.getContext("2d");

    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldWidth * CELL_SIZE;

    gameControlBtn.addEventListener("click", () => {
        const status = world.get_game_status();
        if (status == undefined) {
            gameControlBtn.textContent = "Playing..."
            world.start_game();
            run();
        } else {
            location.reload();
        }
    })

    document.addEventListener("keydown", (event) =>{
        if (world.snake_head_idx() >= 0 && world.snake_head_idx() <= WORLD_WIDTH * WORLD_WIDTH) {
            switch (event.code) {
                case "KeyW":
                    world.change_snake_direction(Direction.Up);
                    break;
                case "KeyS":
                    world.change_snake_direction(Direction.Down);
                    break;
                case "KeyA":
                    world.change_snake_direction(Direction.Left);
                    break;
                case "KeyD":
                    world.change_snake_direction(Direction.Right);
                    break;
            }
        }
    })

    function drawWorld() {
        ctx.beginPath();
        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0);
            ctx.lineTo(CELL_SIZE*x,CELL_SIZE*worldWidth);
        }

        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE*y);
            ctx.lineTo(CELL_SIZE*worldWidth,CELL_SIZE*y);
        }
        ctx.stroke();
    }

    function drawSnake() {
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.get_snake_len(),
        );
        snakeCells
            .filter((idx, i) => !(i > 0 && idx == snakeCells[0]))
            .forEach((cellIdx, i) => {
            const col = cellIdx % worldWidth;
            const row = Math.floor(cellIdx/worldWidth)
            ctx.beginPath();
            ctx.fillStyle = i=== 0? "#787878":"#000000";
            ctx.fillRect(col*CELL_SIZE, row*CELL_SIZE,CELL_SIZE,CELL_SIZE);
        })
        ctx.stroke();
    }

    function drawReward() {
        const idx = world.get_reward_cell();
        const row = Math.floor(idx/worldWidth);
        const col = idx % worldWidth;
        ctx.beginPath();
        ctx.fillStyle = "#ff0000"
        ctx.fillRect(col*CELL_SIZE, row*CELL_SIZE,CELL_SIZE,CELL_SIZE);
        ctx.stroke();
        if (world.get_game_status() === GameStates.Win) {
            alert("Win")
            gameControlBtn.textContent = "Play Again"
        }
    }

    function drawGameStatus() {
        gameStatus.textContent = world.get_game_status_info();
    }

    function draw() {
        drawWorld();
        drawSnake();
        drawReward();
        drawGameStatus();
    }

    function run() {
        const status = world.get_game_status();
        if (status === GameStates.Win || status == GameStates.Lose) {
            gameControlBtn.textContent = "Play Again?";
            return;
        }
        setTimeout(() => {
            ctx.clearRect(0,0,canvas.width, canvas.height)
            world.update();
            draw();
            requestAnimationFrame(run);
        }, 1000/fps)
    }
    draw();
 })

