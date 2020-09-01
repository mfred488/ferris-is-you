const canvas = document.querySelector("#canvas");
const context = canvas.getContext("2d");

const cell_size = 50;
const margin = 0.5;

let width;
let height;
function initLevel(level) {
    width = level.width;
    height = level.height;
    canvas.width = margin +  width * cell_size + margin;
    canvas.height = margin + height * cell_size + margin;
    context.fillStyle = "black";
    context.fillRect(0, 0, canvas.width, canvas.height);
}

function drawGrid() {
    // Draw grid
    for (let x = 0; x <= width ; x++) {
        context.moveTo(margin + x * cell_size, margin);
        context.lineTo(margin + x * cell_size, margin + height * cell_size);
    }
    for (let y = 0; y <= height ; y++) {
        context.moveTo(margin, margin + y * cell_size);
        context.lineTo(margin + width * cell_size, margin + y * cell_size);
    }
    context.strokeStyle = "#696969";
    // context.strokeStyle = "black";
    context.stroke();
}

function drawObject(object, x, y, orientation = "down") {
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.font = '32px sans-serif';
    let angle;
    switch (orientation) {
        case "up":
            angle = Math.PI;
            break;
        case "down":
            angle = 0;
            break;
        case "right":
            angle = -Math.PI / 2;
            break;
        case "left":
            angle = Math.PI / 2;
            break;
    }
    context.rotate(angle);
    const new_x = margin + (x + 0.5) * cell_size;
    const new_y = margin + (y + 0.5) * cell_size;
    const new_location_x = new_x*Math.cos(angle) + new_y*Math.sin(angle);
    const new_location_y = -new_x*Math.sin(angle) + new_y*Math.cos(angle);
    context.fillText(object, new_location_x, new_location_y);
    context.rotate(-angle);
}

function drawAdjective(adjective, x, y) {
    context.globalAlpha = 0.7;
    const cell_orig_x = margin + x * cell_size;
    const cell_orig_y = margin + y * cell_size;
    const internal_padding = 0.09  * cell_size;
    context.fillStyle = "grey";
    context.fillRect(cell_orig_x + internal_padding, cell_orig_y + internal_padding, cell_size - 2 * internal_padding, cell_size - 2 * internal_padding );
    context.font = 'small-caps 900 15px sans-serif'
    context.fillStyle = "black";
    context.fillText(adjective, margin + (x + 0.5) * cell_size, margin + (y + 0.5) * cell_size);
    context.globalAlpha = 1;
}

function drawText(text, x, y) {
    context.globalAlpha = 0.7;
    const cell_orig_x = margin + x * cell_size;
    const cell_orig_y = margin + y * cell_size;
    const internal_padding = 0.09  * cell_size;
    context.fillStyle = "white";
    context.font = 'small-caps 900 15px sans-serif'
    context.fillText(text, margin + (x + 0.5) * cell_size, margin + (y + 0.5) * cell_size);
    context.globalAlpha = 1;
}

// function drawLevel(level) {
//     context.globalAlpha = 0.6;
//     drawObject("🌼", 2, 3);
//     drawObject("🦀", 2, 3, orientation="left");

//     context.globalAlpha = 1;
//     drawObject("🌊", 2, 5);
//     drawObject("🧱", 2, 5);

//     drawObject("🦀", 3, 7);
//     drawText("rock", 4, 5);
//     drawText("is", 4, 6);
//     drawAdjective("sink", 4, 7);

//     drawObject("🌊", 0, 0, orientation="right");
// }
