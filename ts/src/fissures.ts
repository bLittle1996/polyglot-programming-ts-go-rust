function getInput() {
  return `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`;
}

type Point = { x: number; y: number };
type Line = { point1: Point; point2: Point };

function isVerticalLine(line: Line): boolean {
  // If both x values are the same, then the line must go up or down;
  return line.point1.x === line.point2.x && line.point1.y !== line.point2.y;
}
function isHorizontalLine(line: Line): boolean {
  // If both y values are the same, then the line must go left or right;
  return line.point1.y === line.point2.y && line.point1.x !== line.point2.x;
}

function isHoriOrVerti(line: Line): boolean {
  return isVerticalLine(line) || isHorizontalLine(line);
}

function parsePoint(input: string): Point {
  const [x, y] = input.split(",");

  return {
    x: +x,
    y: +y,
  };
}
function parseLine(input: string): Line {
  const [p1, p2] = input.split(" -> ");

  return {
    point1: parsePoint(p1),
    point2: parsePoint(p2),
  };
}

function lineToString(line: Line): string {
  return `(${line.point1.x},${line.point1.y}) -> (${line.point2.x},${line.point2.y})`;
}

function run() {
  const lines = getInput().split("\n").map(parseLine);

  lines.forEach((line) => {
    if (isHoriOrVerti(line)) {
      console.log(`${lineToString(line)} is cardinal`);
    } else {
      console.log(`${lineToString(line)} is not cardinal`);
    }
  });
}

run();
