import("../pkg").then(async module => {
    let board = document.getElementById("board");
    let universe = module.Universe.generate(12, 12);
    board.innerText = universe.toString();
})