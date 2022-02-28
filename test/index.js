import { Prolocube, Pixel } from "prolocube-wasm";
import { memory } from "prolocube-wasm/prolocube_wasm_bg";

let prolocube_width = 400;
const cube = Prolocube.new(prolocube_width, 0, 0, 0, 10);

const canvas = document.getElementById("canvas");
canvas.height = prolocube_width
canvas.width = prolocube_width

const ctx = canvas.getContext('2d');
const arr = new Uint8ClampedArray(prolocube_width * 2);

let roll = 0.1;
let pitch = 0.1;
let yaw = 0.1;

const renderLoop = () => {
    cube.rotate(roll, pitch, yaw);
    const pixelsPtr = cube.render();

    // 4 bytes per pixel and 4 * cube_width^2 pixels
    const cells = new Uint8ClampedArray(memory.buffer, pixelsPtr, prolocube_width * prolocube_width * 4);
    let imageData = new ImageData(cells, prolocube_width);
    ctx.putImageData(imageData, 0, 0);
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
