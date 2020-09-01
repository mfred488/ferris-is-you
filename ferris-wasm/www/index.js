import * as wasm from "ferris-wasm";

let level = wasm.init_level();
initLevel(level);
drawGrid();

const text_capturing_regex = /\(([A-Z]*)\)+$/;
function refresh() {
    initLevel(level);
    drawGrid();
    const grid = level.get_grid()
    for (let x = 0; x < level.width; x++) {
        for (let y = 0; y < level.height; y++) {
            if (grid[level.get_grid_index(x, y)].length > 0) {
                // TODO handle floating object ?
                for (const { orientation, object, unicode } of grid[level.get_grid_index(x, y)]) {
                    if (object.startsWith("Object")) {
                        drawObject(unicode, x, y, orientation);
                    } else {
                        const word = text_capturing_regex.exec(object);
                        if (!word) {
                            console.error(`Unrecognized object ${object}`);
                            continue;
                        }
                        if (object.includes("Adjective")) {
                            drawAdjective(word[1], x, y);
                        } else {
                            drawText(word[1], x, y);
                        }
                    }
                }
            }
        }
    }
}
refresh();

document.addEventListener("keydown", (key) => {
    let input;
    switch (key.code) {
        case "ArrowUp":
            input = "up";
            break;
        case "ArrowDown":
            input = "down";
            break;
        case "ArrowRight":
            input = "right";
            break;
        case "ArrowLeft":
            input = "left";
            break;
        case " ":
            input = "wait";
            break;
        case "Backspace":
            input = "undo";
            break;
        case "r":
        case "R":
            level = wasm.init_level();
        default:
            return;
    }
    if (input) {
        level.next(input)
    };
    refresh();
});

// level.next_right();
// level.next_right();
// level.next_right();
// console.log("***********");
// console.log(level.to_string());


