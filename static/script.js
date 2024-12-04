const boardDiv = document.getElementById('board');
const messageDiv = document.getElementById('message');
const messageText = document.getElementById('message-text');
const restartButton = document.getElementById('restart-button');

restartButton.addEventListener('click', restartGame);

function createBoard(status) {
    boardDiv.innerHTML = '';
    status.board.forEach((row, x) => {
        row.forEach((cell, y) => {
            const cellDiv = document.createElement('div');
            cellDiv.classList.add('cell');
            cellDiv.innerText = cell;
            cellDiv.addEventListener('click', () => makeMove(x, y));
            boardDiv.appendChild(cellDiv);
        });
    });

    if (status.game_over) {
        if (status.winner) {
            messageText.innerText = `A győztes: ${status.winner}`;
        } else {
            messageText.innerText = 'Döntetlen!';
        }
        messageDiv.classList.remove('hidden');
        highlightWinningCells(status.board);
    } else {
        messageDiv.classList.add('hidden');
    }
}

function highlightWinningCells(board) {
    const lines = [

        [[0, 0], [0, 1], [0, 2]],
        [[1, 0], [1, 1], [1, 2]],
        [[2, 0], [2, 1], [2, 2]],

        [[0, 0], [1, 0], [2, 0]],
        [[0, 1], [1, 1], [2, 1]],
        [[0, 2], [1, 2], [2, 2]],

        [[0, 0], [1, 1], [2, 2]],
        [[0, 2], [1, 1], [2, 0]],
    ];

    lines.forEach(line => {
        const [a, b, c] = line;
        const cellA = board[a[0]][a[1]];
        const cellB = board[b[0]][b[1]];
        const cellC = board[c[0]][c[1]];

        if (cellA && cellA === cellB && cellA === cellC) {
            const cells = document.querySelectorAll('.cell');
            line.forEach(([x, y]) => {
                const index = x * 3 + y;
                cells[index].classList.add('winner');
            });
        }
    });
}

async function getStatus() {
    const response = await fetch('/status');
    const status = await response.json();
    createBoard(status);
}

async function makeMove(x, y) {
    const response = await fetch('/move', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ x, y })
    });
    if (response.ok) {
        getStatus();
    } else {
        const errorText = await response.text();
        alert(errorText);
    }
}

async function restartGame() {
    await fetch('/restart', {
        method: 'POST'
    });
    getStatus();
}

getStatus();
