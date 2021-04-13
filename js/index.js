function drawImage(image) {
    const canvas = document.getElementById('mycanvas');
    const context = canvas.getContext('2d');
    context.strokeStyle = 'black';
    context.lineWidth = 1;

    const width = image.width();
    const height = image.height();
    const cellSize = 50;

    const cells = image.cells();
    for(let x=0; x< width; x++) {
        for(let y=0; y< width; y++) {
            const index = (y * width + x) * 3;
            const color = `rgb(${cells[index+0]}, ${cells[index+1]}, ${cells[index+2]})`;
            context.fillStyle = color;
            context.fillRect(x* cellSize, y*cellSize, cellSize, cellSize);
        }
    }

    for(let x=0; x<= width; x++) {
        context.beginPath();
        context.moveTo(x*cellSize+0.5, 0);
        context.lineTo(x*cellSize+0.5, height*cellSize);
        context.stroke();
    }

    for(let y=0; y<= height; y++) {
        context.beginPath();
        context.moveTo(0, y*cellSize+0.5);
        context.lineTo(width*cellSize, y*cellSize+0.5);
        context.stroke();
    }
}

async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);
    const image = new lib.Image(20, 20);
    drawImage(image);
}

main();
