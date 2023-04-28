const input = `forward 5
down 5
forward 8
up 3
down 8
forward 2`;

console.log(processInput(input));

function processInput(input: string): { coodinate: Coordinate, answer: number } {
    const finalCoordinate = input.split("\n").map(commandToCoordinate).reduce((summedCoordinate, coordinate) => {
        summedCoordinate[0] = summedCoordinate[0] + coordinate[0];
        summedCoordinate[1] = summedCoordinate[1] + coordinate[1];

        return summedCoordinate;
    }, [0, 0])

    return {
        coodinate: finalCoordinate,
        answer: finalCoordinate[0] * finalCoordinate[1]
    }
}

/** 
* [X, Y]
* Y represents _depth_. As it increases, the deeper you are. A negative number means you are going up.
*/
type Coordinate = [number, number];

function commandToCoordinate(command: string): Coordinate {
    const [direction, amount] = command.split(" ");

    switch(direction) {
        case "up":
            return [0, toNumber(amount) * -1];
        case "down":
            return [0, toNumber(amount)];
        case "forward":
            return [toNumber(amount), 0];
        case "backward":
            return [toNumber(amount) * -1, 0];
        default:
            return [0, 0];
    }
}

function toNumber(stringNumber: string): number {
    const asNumber = Number.parseInt(stringNumber);

    if(Number.isNaN(asNumber)) {
        return 0;
    }

    return asNumber;
}
