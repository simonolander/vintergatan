function renderBoard(universe, board) {
    board.innerHTML = '';
    const wallCellRatio = 0.1;
    let universeSize = universe.width;
    const cellWidth = board.clientWidth / (universeSize + (universeSize + 1) * wallCellRatio);
    const wallWidth = cellWidth * wallCellRatio;

    for (let row = 0; row < universeSize; row += 1) {
        let y1 = wallWidth / 2 + (wallWidth + cellWidth) * row;
        let y2 = wallWidth / 2 + (wallWidth + cellWidth) * (row + 1);
        for (let col = 0; col < universeSize - 1; col++) {
            const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
            let x = wallWidth / 2 + (wallWidth + cellWidth) * (col + 1);
            line.setAttribute("x1", x)
            line.setAttribute("y1", y1)
            line.setAttribute("x2", x)
            line.setAttribute("y2", y2)
            line.setAttribute("stroke", "#5a5a5a");
            line.setAttribute("stroke-width", wallWidth);
            line.setAttribute("stroke-linecap", "round");
            board.appendChild(line);
        }
    }

    for (let col = 0; col < universeSize; col += 1) {
        let x1 = wallWidth / 2 + (wallWidth + cellWidth) * col;
        let x2 = wallWidth / 2 + (wallWidth + cellWidth) * (col + 1);
        for (let row = 0; row < universeSize - 1; row++) {
            const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
            let y = wallWidth / 2 + (wallWidth + cellWidth) * (row + 1);
            line.setAttribute("x1", x1)
            line.setAttribute("y1", y)
            line.setAttribute("x2", x2)
            line.setAttribute("y2", y)
            line.setAttribute("stroke", "#5a5a5a");
            line.setAttribute("stroke-width", wallWidth);
            line.setAttribute("stroke-linecap", "round");
            board.appendChild(line);
        }
    }

    const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("x", wallWidth / 2);
    rect.setAttribute("y", wallWidth / 2);
    rect.setAttribute("width", board.clientWidth - wallWidth);
    rect.setAttribute("height", board.clientWidth - wallWidth);
    rect.setAttribute("stroke", "#5a5a5a");
    rect.setAttribute("stroke-width", wallWidth);
    rect.setAttribute("fill", "none");
    board.appendChild(rect);
}

import("../pkg").then(async module => {
    let board = document.getElementById("board");
    let universe = module.Universe.generate(10, 10);
    board.innerText = universe.toString();
    renderBoard(universe, board);
})