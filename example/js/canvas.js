const canvas = document.getElementById('prolocube');

let width = canvas.offsetWidth;
let height = canvas.offsetHeight;

const ctx = canvas.getContext('2d');

let down = false;
let last = 5;
let previous_angle = [];
let curr_pos = [];
let angle = [0, 0];

canvas.addEventListener('mouseup', e => {
    if (down === true) {
        let first_angle = previous_angle[0];
        let last_angle = previous_angle[previous_angle.length - 1];
        let delta = [last_angle[0] - first_angle[0], last_angle[1] - first_angle[1]];

        // while (last != 0) {
        //     last = last - 1;

        //     let angle_x_dir = 0;
        //     if (delta[0] < 0) { angle_x_dir = -1; }
        //     else if (delta[0] > 0) { angle_x_dir = 1; }
        //     angle[0] -= angle_x_dir;

        //     let angle_y_dir = 0;
        //     if (delta[1] < 0) { angle_y_dir = -1; }
        //     else if (delta[1] > 0) { angle_y_dir = 1; }
        //     angle[1] -= angle_y_dir;
        // }
    }
    previous_angle = [];
    down = false;
});

canvas.addEventListener('mousedown', e => {
    last = 5;
    down = true;
    curr_pos = [e.offsetX, e.offsetY];
});

canvas.addEventListener('mousemove', e => {
    if (down === true) {
        if (previous_angle.length > 10) {
            previous_angle.shift();
        }
        console.log(angle);
        previous_angle.push([e.offsetX, e.offsetY]);
        let angle_offset = [curr_pos[0] - e.offsetX, curr_pos[1] - e.offsetY];
        curr_pos = [e.offsetX, e.offsetY];
        angle = [(angle[0] + angle_offset[0] + 360) % 360,
                 (angle[1] + angle_offset[1] + 360) % 360];
    }
});

function draw() {
    // ctx.fillStyle = 'rgba(255,255,255,1.0)';
    // ctx.fillRect(0,0,width,height);
    // ctx.beginPath();
    // ctx.arc(angle[0], angle[1], 6, 0, Math.PI*2, true);
    // ctx.closePath();
    // ctx.fillStyle = 'black';
    // ctx.fill();
    const arr = new Uint8ClampedArray(40000);
    let imageData = new ImageData(arr, 200);
    ctx.putImageData(imageData, 20, 20);
    window.requestAnimationFrame(draw);
}

window.requestAnimationFrame(draw);
