import * as wasm from "../pkg/webgltest.js"
async function main() {
    const wasmBackend = await wasm.default();
    const WIDTH = 900, HEIGHT = WIDTH;

    let c = document.createElement("canvas");
    c.width = WIDTH;
    c.style.border = "solid 1px black";
    c.height = HEIGHT;
    document.body.append(c);
    let graphics = c.getContext("2d")!;
    let wasmWindow = wasm.GameWindow.new(WIDTH, HEIGHT);


    
    
    let averageTime = 0;
    let fps = 0;
    async function gameLoop() {
        while (true) {
            let start = performance.now();
            wasm.render(wasmWindow);
            let end = performance.now();
            averageTime+= end-start;

            let pixels = new Uint8ClampedArray(wasmBackend.memory.buffer, wasmWindow.bufferPtr(),WIDTH*HEIGHT*4);
            let imageData = new ImageData(pixels, WIDTH, HEIGHT);
            graphics.putImageData(imageData, 0, 0);
            await sleep(1000 / 15);
            fps++ 
        }
    }

    async function fpsPrinter() {
        while (true) {
            await sleep(1000);
            console.log("FPS: " + Math.round(fps/10)*10 + " time: " + averageTime / fps);
            fps = 0;
            averageTime = 0;
        }
    }
    gameLoop()
    //fpsPrinter()
}


async function sleep(sec: number) {
    return new Promise(resolve => setTimeout(resolve, sec));
}

main()
